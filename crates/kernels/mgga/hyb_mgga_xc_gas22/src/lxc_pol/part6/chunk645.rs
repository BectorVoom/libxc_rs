//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 645/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk645<F: Float>(t1252: F, t154: F, t3188: F, t712: F, t157: F, t716: F, t160: F, t720: F, t163: F, t724: F, t166: F, t728: F, t169: F, t732: F, t2098: F, t736: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3191 = t154 * t1252;
    let t3194 = t712 * t3188;
    let t3196 = t157 * t1252;
    let t3199 = t716 * t3188;
    let t3201 = t160 * t1252;
    let t3204 = t720 * t3188;
    let t3206 = t163 * t1252;
    let t3209 = t724 * t3188;
    let t3211 = t166 * t1252;
    let t3214 = t728 * t3188;
    let t3216 = t169 * t1252;
    let t3219 = t732 * t3188;
    let t3221 = t2098 * t1252;
    let t3224 = t736 * t3188;
    (t3191, t3194, t3196, t3199, t3201, t3204, t3206, t3209, t3211, t3214, t3216, t3219, t3221, t3224)
}
