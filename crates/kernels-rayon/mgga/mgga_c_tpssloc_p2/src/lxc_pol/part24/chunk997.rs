//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 997/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk997(t3402: f64, t448: f64, t11129: f64, t11282: f64, t1164: f64, t3411: f64, t3415: f64, t3399: f64, t445: f64, t3403: f64, t1143: f64, t3375: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11285 = 1.0_f64 / t3402 / t448;
    let t11286 = t11282 * t11129 * t11285;
    let t11288 = 0.10254018858216406658e4_f64 * t1164 * t11286;
    let t11290 = 0.35089341735807877242e1_f64 * t3411 * t3415;
    let t11292 = 1.0_f64 / t3399 / t445;
    let t11294 = t11292 * t11129 * t3403;
    let t11296 = 0.10389515463408878255e3_f64 * t1164 * t11294;
    let t11297 = t1143 * t3375;
    (t11285, t11288, t11290, t11292, t11296, t11297)
}
