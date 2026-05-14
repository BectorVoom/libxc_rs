//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 600/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk600<F: Float>(t1104: F, t3067: F, t1085: F, t1094: F, t3053: F, t1102: F, t2917: F, t3058: F, t3061: F, t116: F, t2837: F, t428: F, t1115: F, t24: F, t1111: F, t1027: F, t371: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3069 = 0.11696446794910408142e1 * t3067 * t1104;
    let t3071 = t1085 * t3053 * t1094;
    let t3073 = 0.58482233974552040708e0 * t1102 * t3071;
    let t3074 = t3058 * t2917;
    let t3075 = t3074 * t3061;
    let t3077 = 0.17315755899375863299e2 * t1102 * t3075;
    let t3079 = t116 * t2837;
    let t3081 = t428 * t3079 / 432.0;
    let t3082 = t24 * t1115;
    let t3083 = t1111 * t3082;
    let t3086 = 1.0 / t371 / t1027;
    (t3069, t3071, t3073, t3074, t3075, t3077, t3081, t3083, t3086)
}
