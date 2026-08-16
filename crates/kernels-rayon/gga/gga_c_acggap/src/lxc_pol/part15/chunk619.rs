//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 619/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk619(t1775: f64, t330: f64, t1165: f64, t1889: f64, t407: f64, t1894: f64, t1181: f64, t1899: f64, t1439: f64, t4643: f64, t372: f64, t960: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5884 = t330 * t1775;
    let t5891 = t1165 * t1889 * t407;
    let t5894 = t1894 * t407;
    let t5895 = t1181 * t5894;
    let t5899 = t1165 * t1899 * t407;
    let t5902 = t4643 * t1439;
    let t5903 = t1181 * t5902;
    let t5906 = t1889 * t372;
    let t5907 = t960 * t5906;
    (t5884, t5891, t5895, t5899, t5903, t5906, t5907)
}
