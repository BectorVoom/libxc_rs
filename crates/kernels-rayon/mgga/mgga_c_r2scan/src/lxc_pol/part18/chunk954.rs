//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 954/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk954(t11479: f64, t3275: f64, t3352: f64, t3270: f64, t3618: f64, t3269: f64, t3574: f64, t792: f64, t3276: f64, t3262: f64, t10918: f64, t3263: f64, t7040: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11481 = t3275 * t11479 * t3352;
    let t11482 = t11481 / 4.0_f64;
    let t11483 = t3270 * t3618;
    let t11484 = t3269 * t11483;
    let t11485 = t11484 / 4.0_f64;
    let t11486 = t3574 * t792;
    let t11487 = t3276 * t11486;
    let t11488 = t3262 * t11487;
    let t11489 = 15.0_f64 / 16.0_f64 * t11488;
    let t11491 = t3262 * t10918 * t3574;
    let t11492 = 3.0_f64 / 4.0_f64 * t11491;
    let t11494 = t3275 * t3263 * t7040;
    (t11481, t11482, t11483, t11484, t11485, t11486, t11487, t11488, t11489, t11491, t11492, t11494)
}
