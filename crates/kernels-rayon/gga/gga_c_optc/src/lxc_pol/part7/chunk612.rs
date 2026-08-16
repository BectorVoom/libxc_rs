//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 612/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk612(t2917: f64, t3061: f64, t1067: f64, t1076: f64, t1086: f64, t1095: f64, t2927: f64, t2930: f64, t2935: f64, t2937: f64, t2969: f64, t2974: f64, t2977: f64, t2987: f64, t2990: f64, t2997: f64, t3015: f64, t3023: f64, t3030: f64, t3032: f64, t3035: f64, t3036: f64, t3054: f64, t3059: f64, t402: f64) -> (f64, f64) {
    let t3062 = t2917 * t3061;
    let t3065 = -0.3109e-1_f64 * t2927 * t402 + 2.0_f64 * t2930 * t1076 - 2.0_f64 * t2935 * t2937 + 1.0_f64 * t1067 * t2969 + 0.32164683177870697974e2_f64 * t2974 * t2977 + t2987 - t2990 + t2997 - t3015 - t3023 - 0.19751789702565206229e-1_f64 * t3030 + 0.11696446794910408142e1_f64 * t3032 * t1095 - 0.11696446794910408142e1_f64 * t3035 * t3036 + 0.58482233974552040708e0_f64 * t1086 * t3054 + 0.17315755899375863299e2_f64 * t3059 * t3062;
    (t3062, t3065)
}
