//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1197/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1197(t106: f64, t7194: f64, t97: f64, t3271: f64, t10615: f64, t11531: f64, t3275: f64, t3579: f64, t37257: f64, t11621: f64, t37292: f64, t11560: f64, t37271: f64) -> (f64, f64, f64, f64, f64) {
    let t40358 = t97 * t106 * t7194;
    let t40360 = t40358 * t3271 / 4.0_f64;
    let t40363 = 5.0_f64 / 8.0_f64 * t3275 * t10615 * t11531;
    let t40365 = 5.0_f64 / 8.0_f64 * t3579 * t37257;
    let t40368 = 45.0_f64 / 32.0_f64 * t3275 * t37292 * t11621;
    let t40370 = 5.0_f64 / 8.0_f64 * t37271 * t11560;
    (t40360, t40363, t40365, t40368, t40370)
}
