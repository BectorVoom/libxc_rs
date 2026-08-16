//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 922/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk922(t1085: f64, t1094: f64, t8738: f64, t1102: f64, t241: f64, t3029: f64, t1104: f64, t2919: f64, t3067: f64, t3057: f64, t411: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8740 = t1085 * t8738 * t1094;
    let t8742 = 0.58482233974552040708e0_f64 * t1102 * t8740;
    let t8743 = t241 * t3029;
    let t8745 = 0.17544670192365612213e1_f64 * t8743 * t1104;
    let t8747 = 0.35089340384731224426e1_f64 * t3067 * t2919;
    let t8749 = 1.0_f64 / t3057 / t411;
    (t8740, t8742, t8743, t8745, t8747, t8749)
}
