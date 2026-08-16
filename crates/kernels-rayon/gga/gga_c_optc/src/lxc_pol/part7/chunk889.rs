//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 889/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk889(t1094: f64, t3058: f64, t8553: f64, t1102: f64, t1032: f64, t2992: f64, t2995: f64, t1055: f64, t2994: f64, t1056: f64, t3018: f64, t1057: f64, t3012: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8555 = t3058 * t8553 * t1094;
    let t8557 = 0.35089340384731224426e1_f64 * t1102 * t8555;
    let t8558 = t1032 * t2992;
    let t8560 = 6.0_f64 * t8558 * t2995;
    let t8561 = t2994 * t1055;
    let t8562 = t8561 * t1056;
    let t8564 = 6.0_f64 * t3018 * t8562;
    let t8565 = t1057 * t3012;
    (t8555, t8557, t8558, t8560, t8561, t8562, t8564, t8565)
}
