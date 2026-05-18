//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 560/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk560<F: Float>(t322: F, t3307: F, t1069: F, t913: F, t1072: F, t1074: F, t869: F, t3272: F, t910: F, t1084: F, t3072: F, t1087: F, t3074: F) -> (F, F, F, F, F, F, F) {
    let t3308 = t3307 * t322;
    let t3310 = t1069 * t913;
    let t3314 = t869 * t1072 * t1074;
    let t3316 = t3272 * t1074;
    let t3318 = t1069 * t910;
    let t3320 = t1084 * t3072;
    let t3321 = t1087 * t3074;
    (t3308, t3310, t3314, t3316, t3318, t3320, t3321)
}
