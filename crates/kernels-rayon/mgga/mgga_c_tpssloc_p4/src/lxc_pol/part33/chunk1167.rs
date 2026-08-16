//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1167/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1167(t22960: f64, t28248: f64, t1408: f64, t1484: f64, t25: f64, t5544: f64, t5657: f64, t6571: f64, t6553: f64, t1880: f64, t1527: f64, t25191: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28249 = t22960 * t28248;
    let t28252 = t1408 * t1484;
    let t28256 = t25 * t5544;
    let t28263 = t6571 * t5657;
    let t28264 = t6553 * t28263;
    let t28265 = t1880 * t28264;
    let t28267 = t25191 * t1527;
    (t28249, t28252, t28256, t28263, t28264, t28265, t28267)
}
