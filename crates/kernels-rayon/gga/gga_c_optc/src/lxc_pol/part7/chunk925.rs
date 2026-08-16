//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 925/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk925(t1093: f64, t8791: f64, t1076: f64, t2968: f64, t1074: f64, t2976: f64, t1095: f64, t3053: f64, t1086: f64, t2930: f64, t2935: f64, t2969: f64, t2974: f64, t2977: f64, t3032: f64, t3035: f64, t3036: f64, t3054: f64, t3059: f64, t3062: f64, t8567: f64, t8754: f64, t8757: f64, t8762: f64, t8765: f64, t8766: f64, t8769: f64, t8772: f64, t8773: f64, t8776: f64, t8781: f64, t8786: f64, t8788: f64) -> (f64, f64, f64, f64, f64) {
    let t8792 = t8791 * t1093;
    let t8795 = t1076 * t2968;
    let t8799 = t2968 * t2976 * t1074;
    let t8802 = t1095 * t3053;
    let t8805 = -0.35089340384731224426e1_f64 * t8754 * t3036 + 0.35089340384731224426e1_f64 * t3059 * t8757 + 0.17544670192365612213e1_f64 * t3032 * t3054 + 0.51947267698127589899e2_f64 * t8762 * t3062 - 0.1038945353962551798e3_f64 * t8765 * t8766 + 0.58482233974552040708e0_f64 * t1086 * t8769 + 0.1025389702100779493e4_f64 * t8772 * t8773 + 3.0_f64 * t8776 * t1076 + 3.0_f64 * t2930 * t2969 + 0.96494049533612093922e2_f64 * t8781 * t2977 - 0.19298809906722418785e3_f64 * t8786 * t8788 + 0.51947267698127589897e2_f64 * t3059 * t8792 - 6.0_f64 * t2935 * t8795 + 0.96494049533612093922e2_f64 * t2974 * t8799 - 0.35089340384731224426e1_f64 * t3035 * t8802 + t8567;
    (t8792, t8795, t8799, t8802, t8805)
}
