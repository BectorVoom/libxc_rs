//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 650/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk650(t1137: f64, t1867: f64, t145: f64, t1713: f64, t301: f64, t960: f64, t1884: f64, t372: f64, t1298: f64, t1313: f64, t1734: f64, t1753: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6371 = t1137 * t1867;
    let t6374 = t145 * t1713;
    let t6375 = t6374 * t301;
    let t6376 = t960 * t6375;
    let t6379 = t1884 * t372;
    let t6380 = t960 * t6379;
    let t6383 = t1313 * t1298;
    let t6384 = t960 * t6383;
    let t6387 = t145 * t1734;
    let t6388 = t6387 * t301;
    let t6389 = t960 * t6388;
    let t6394 = t1753 * t372;
    (t6371, t6375, t6376, t6379, t6380, t6383, t6384, t6388, t6389, t6394)
}
