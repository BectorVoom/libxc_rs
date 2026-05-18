//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 757/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk757<F: Float>(t1378: F, t7274: F, t930: F, t11073: F, t953: F, t322: F, t3882: F, t3881: F, t1382: F, t864: F, t116: F, t2718: F, t2719: F) -> (F, F, F, F, F, F) {
    let t11191 = t7274 * t1378;
    let t11192 = t930 * t11191;
    let t11199 = t953 * t11073;
    let t11325 = t3882 * t322;
    let t11326 = t3881 * t11325;
    let t11327 = t864 * t1382;
    let t11368 = t2718 * t2719 * t116;
    (t11192, t11199, t11325, t11326, t11327, t11368)
}
