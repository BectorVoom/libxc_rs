//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1225/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1225(t10940: f64, t11483: f64, t3275: f64, t3582: f64, t37543: f64, t11855: f64, t1561: f64, t3277: f64, t3262: f64, t3574: f64, t37318: f64, t113: f64, t3578: f64, t97: f64) -> (f64, f64, f64, f64, f64) {
    let t40699 = t10940 * t11483 / 4.0_f64;
    let t40704 = 5.0_f64 / 16.0_f64 * t3275 * t37543 * t3582;
    let t40705 = t1561 * t11855;
    let t40708 = 5.0_f64 / 8.0_f64 * t3275 * t40705 * t3277;
    let t40711 = 3.0_f64 / 4.0_f64 * t3262 * t37318 * t3574;
    let t40713 = t97 * t3578 * t113;
    (t40699, t40704, t40708, t40711, t40713)
}
