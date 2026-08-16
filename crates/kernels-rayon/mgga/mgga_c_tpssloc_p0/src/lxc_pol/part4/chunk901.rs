//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 901/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk901(t3314: f64, t422: f64, t1146: f64, t3399: f64, t3402: f64, t448: f64, t445: f64, t1143: f64, t3375: f64, t1124: f64, t3331: f64, t440: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11277 = 1.0_f64 / t3314 / t422;
    let t11282 = 1.0_f64 / t3399 / t1146;
    let t11285 = 1.0_f64 / t3402 / t448;
    let t11292 = 1.0_f64 / t3399 / t445;
    let t11297 = t1143 * t3375;
    let t11303 = t1124 * t3331;
    let t11310 = t440 * t11282;
    (t11277, t11282, t11285, t11292, t11297, t11303, t11310)
}
