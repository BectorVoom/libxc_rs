//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 342/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk342(t1085: f64, t1093: f64, t1094: f64, t1102: f64, t23: f64, t429: f64, t116: f64, t428: f64, t427: f64, t861: f64) -> (f64, f64, f64, f64, f64) {
    let t1104 = t1085 * t1093 * t1094;
    let t1106 = 0.58482233974552040708e0_f64 * t1102 * t1104;
    let t1107 = t23 * t429;
    let t1108 = t116 * t1107;
    let t1110 = t428 * t1108 / 288.0_f64;
    let t1111 = t427 * t861;
    (t1104, t1106, t1107, t1110, t1111)
}
