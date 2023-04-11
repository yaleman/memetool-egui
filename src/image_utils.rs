use std::path::PathBuf;

use eframe::egui;
use eframe::epaint::{ColorImage, Vec2};
use egui_extras::RetainedImage;
use log::*;

use crate::THUMBNAIL_SIZE;

pub fn load_image_to_thumbnail(
    filename: &PathBuf,
    size: Option<Vec2>,
) -> Result<RetainedImage, String> {
    debug!("Loading {}", filename.to_string_lossy());
    puffin::profile_function!(filename.display().to_string());
    let image = image::io::Reader::open(filename)
        .map_err(|e| e.to_string())?
        .decode()
        .map_err(|e| e.to_string())?;

    let (x, y) = match size {
        Some(size) => (size.x as u32, size.y as u32),
        None => (THUMBNAIL_SIZE.x as u32, THUMBNAIL_SIZE.y as u32),
    };

    let image = image.thumbnail(x, y);

    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();

    let ci = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

    let response = egui_extras::RetainedImage::from_color_image(filename.to_string_lossy(), ci);
    debug!("Finished loading {}", filename.display());
    Ok(response)
}

/// throw some pixels at it, get a texture back
pub fn load_image_from_memory(image_data: &[u8]) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::load_from_memory(image_data)?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}