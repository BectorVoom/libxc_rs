//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1163/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1163(t10918: f64, t12570: f64, t3262: f64, t10610: f64, t12414: f64, t3275: f64, t8601: f64, t114: f64, t481: f64, t97: f64, t12415: f64, t2847: f64, t3574: f64) -> (f64, f64, f64, f64, f64) {
    let t42908 = 3.0_f64 / 4.0_f64 * t3262 * t10918 * t12570;
    let t42911 = 3.0_f64 / 2.0_f64 * t10610 * t10918 * t12414;
    let t42914 = t3275 * t10918 * t8601 / 4.0_f64;
    let t42916 = t97 * t481 * t114;
    let t42918 = 3.0_f64 / 2.0_f64 * t42916 * t12415;
    let t42919 = t3574 * t2847;
    (t42908, t42911, t42914, t42918, t42919)
}
