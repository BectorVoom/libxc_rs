//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 614/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk614(t1104: f64, t3067: f64, t1085: f64, t1094: f64, t3053: f64, t1102: f64, t2917: f64, t3058: f64, t3061: f64, t116: f64, t2837: f64, t428: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3069 = 0.11696446794910408142e1_f64 * t3067 * t1104;
    let t3071 = t1085 * t3053 * t1094;
    let t3073 = 0.58482233974552040708e0_f64 * t1102 * t3071;
    let t3074 = t3058 * t2917;
    let t3075 = t3074 * t3061;
    let t3077 = 0.17315755899375863299e2_f64 * t1102 * t3075;
    let t3079 = t116 * t2837;
    let t3081 = t428 * t3079 / 432.0_f64;
    (t3069, t3071, t3073, t3074, t3075, t3077, t3081)
}
