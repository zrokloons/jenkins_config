use serde::{Deserialize};
use std::{fs};


#[derive(Deserialize)]
pub struct JenkinsJob {

    // url of Jenkins instance including port
    pub jenkins: String,

    // name to be shown in the bar
    pub name: String,

    // list of names of jobs that should be tracked
    pub jobs: Vec<String>,
}

#[derive(Deserialize)]
pub struct JenkinsConfig {
    // List of Jenkins jobs
    pub jobs: Vec<JenkinsJob>,

    // Update frequency (seconds)
    pub update_frequency: u64,
}

impl JenkinsConfig {

    // Create a JenkinsWidget struct
    pub fn new(path: &str) -> JenkinsConfig {

        // Read the config into a String
        let data = match fs::read_to_string(&path) {
            Ok(data) => data,
            Err(error) => {
                panic!("Error reading file: {}, {}", path, error);
            },
        };

        // deserialize contents into JenkinsWidget struct
        match serde_yaml::from_str(&data) {
            Ok(jenkins_widget) => jenkins_widget,
            Err(error) => {
                panic!("Error deserialize config: {}, {}", path, error);
            },
        }
    }
}
