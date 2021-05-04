type Mat = Vec<Vec<f64>>;

fn mean(v: &Vec<f64>) -> f64 {
    v.iter().fold(0.0, |a, b| a + b) / (v.len() as f64)
}

fn variance(v: &Vec<f64>) -> f64 {
    let m = mean(v);
    mean(&v.iter().map(|a| (a - m) * (a - m)).collect()).sqrt()
}

pub fn covariance(v1: &Vec<f64>, v2: &Vec<f64>) -> f64 {
    if (v1.len() != v2.len()) {
        panic!("2 vector length are not same");
    }
    let m1 = mean(v1);
    let m2 = mean(v2);
    mean(&(0..(v1.len())).map(|i| (v1[i] - m1) * (v2[i] - m2)).collect())
}

pub fn covariance_matrix(m: &Mat) -> Mat {
    let n = m.len();
    for i in 0..m.len() {
        if n != m[i].len() {
            panic!("matrix shape is invalid");
        }
    }
    (0..n).map(|i| (0..n).map(|j| covariance(&m[i], &m[j])).collect()).collect()
}

#[cfg(test)]
mod test {
    use super::covariance;
    #[test]
    fn all_same_variance() {
        let v = (0..10).map(|_| 0.0).collect();
        assert_eq!(covariance(&v, &v), 0.0);
    }

    #[test]
    fn cascade_variance() {
        let v = (0..5).map(|n| n as f64).collect();
        assert_eq!(covariance(&v, &v), 2.0_f64);
    }
}
