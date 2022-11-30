use csv::Reader;
use egui::Grid;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub struct GridFromCSV {
    pub grid: Grid,
    pub path: String,
}
impl GridFromCSV {
    pub fn new(path: String) -> Self {
        GridFromCSV { grid: Grid::new("some_unique_id"), path: path }
    }
}

pub fn add_two(x: i32) -> i32 {
    x + 2
}