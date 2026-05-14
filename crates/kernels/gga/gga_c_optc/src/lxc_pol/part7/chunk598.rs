//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 598/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk598<F: Float>(t2917: F, t3061: F, t1067: F, t1076: F, t1086: F, t1095: F, t2927: F, t2930: F, t2935: F, t2937: F, t2969: F, t2974: F, t2977: F, t2987: F, t2990: F, t2997: F, t3015: F, t3023: F, t3030: F, t3032: F, t3035: F, t3036: F, t3054: F, t3059: F, t402: F) -> (F, F) {
    let t3062 = t2917 * t3061;
    let t3065 = -0.3109e-1 * t2927 * t402 + 2.0 * t2930 * t1076 - 2.0 * t2935 * t2937 + 1.0 * t1067 * t2969 + 0.32164683177870697974e2 * t2974 * t2977 + t2987 - t2990 + t2997 - t3015 - t3023 - 0.19751789702565206229e-1 * t3030 + 0.11696446794910408142e1 * t3032 * t1095 - 0.11696446794910408142e1 * t3035 * t3036 + 0.58482233974552040708e0 * t1086 * t3054 + 0.17315755899375863299e2 * t3059 * t3062;
    (t3062, t3065)
}
