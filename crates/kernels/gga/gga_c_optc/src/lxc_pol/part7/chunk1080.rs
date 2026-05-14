//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1080/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1080<F: Float>(t10959: F, t3835: F, t7359: F, t7433: F, t875: F, t10888: F, t2678: F, t2669: F, t7835: F, t10: F, t2666: F, t2662: F, t7495: F, t2246: F, t2661: F, t2667: F) -> (F, F, F, F, F, F, F) {
    let t24492 = t3835 * t10959 * t7359;
    let t24494 = t7433 * t875;
    let t24496 = t2678 * t24494 * t10888;
    let t24498 = t2669 * t7835;
    let t24502 = t2666 * t10;
    let t24503 = t2662 * t24502;
    let t24504 = t24503 * t7495;
    let t24507 = t2661 * t2246 * t2667;
    (t24492, t24494, t24496, t24498, t24502, t24504, t24507)
}
