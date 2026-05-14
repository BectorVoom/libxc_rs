//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 913/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk913<F: Float>(t322: F, t3882: F, t3881: F, t116: F, t2718: F, t2719: F, t2263: F, t8384: F, t2269: F, t2434: F, t2748: F, t883: F, t2667: F, t2769: F, t852: F, t3883: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11325 = t3882 * t322;
    let t11326 = t3881 * t11325;
    let t11368 = t2718 * t2719 * t116;
    let t11369 = t8384 * t2263;
    let t11374 = t2434 * t2269;
    let t11398 = t2748 * t883;
    let t11399 = t11398 * t2667;
    let t11450 = t2769 * t852;
    let t11451 = t11450 * t3883;
    (t11325, t11326, t11368, t11369, t11374, t11398, t11399, t11450, t11451)
}
