//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1135/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1135(t3263: f64, t3275: f64, t42403: f64, t12574: f64, t37292: f64, t3262: f64, t3574: f64, t40324: f64, t106: f64, t8691: f64, t97: f64, t3271: f64) -> (f64, f64, f64, f64) {
    let t42405 = t3275 * t3263 * t42403;
    let t42408 = 45.0_f64 / 64.0_f64 * t3275 * t37292 * t12574;
    let t42411 = 3.0_f64 / 2.0_f64 * t3262 * t40324 * t3574;
    let t42413 = t97 * t106 * t8691;
    let t42415 = t42413 * t3271 / 4.0_f64;
    (t42405, t42408, t42411, t42415)
}
