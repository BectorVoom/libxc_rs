//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 645/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk645(t301: f64, t6288: f64, t960: f64, t1899: f64, t372: f64, t1866: f64, t3282: f64, t1567: f64, t513: f64, t1524: f64, t530: f64, t1782: f64, t435: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6289 = t6288 * t301;
    let t6290 = t960 * t6289;
    let t6293 = t1899 * t372;
    let t6294 = t960 * t6293;
    let t6297 = t3282 * t1866;
    let t6300 = t1567 * t513;
    let t6301 = t960 * t6300;
    let t6304 = t530 * t1524;
    let t6305 = t960 * t6304;
    let t6308 = t435 * t1782;
    (t6289, t6290, t6293, t6294, t6297, t6300, t6301, t6304, t6305, t6308)
}
