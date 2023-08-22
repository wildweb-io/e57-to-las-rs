use e57::CartesianCoordinate;
use serde::Serialize;
extern crate rayon;

use anyhow::Result;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Write as IoWrite},
    path::Path,
    sync::Mutex,
};

#[derive(Serialize)]
pub struct StationPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub(crate) fn create_station_point(sum_coordinate: (f64, f64, f64), count: f64) -> StationPoint {
    StationPoint {
        x: sum_coordinate.0 / count,
        y: sum_coordinate.1 / count,
        z: sum_coordinate.2 / count,
    }
}

pub(crate) fn create_station_file(
    output_path: String,
    stations: Mutex<HashMap<usize, StationPoint>>,
) -> Result<()> {
    let stations_file = File::create(Path::new(&output_path).join("stations.json"))?;
    let mut writer = BufWriter::new(stations_file);
    serde_json::to_writer(&mut writer, &stations)?;
    writer.flush()?;

    Ok(())
}

pub(crate) fn get_sum_coordinates(
    sum_coordinates: (f64, f64, f64),
    point: &e57::Point,
) -> (f64, f64, f64) {
    if let CartesianCoordinate::Valid { x, y, z } = point.cartesian {
        (
            sum_coordinates.0 + x,
            sum_coordinates.1 + y,
            sum_coordinates.2 + z,
        )
    } else {
        sum_coordinates
    }
}
