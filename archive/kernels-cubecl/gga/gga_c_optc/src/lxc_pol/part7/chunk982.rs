//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 982/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk982<F: Float>(t11024: F, t935: F, t2752: F, t106: F, t902: F, t322: F, t3882: F, t3881: F, t116: F, t2718: F, t2719: F, t2263: F, t8384: F) -> (F, F, F, F, F, F, F) {
    let t11025 = t11024 * t935;
    let t11029 = t2752 * t935;
    let t11140 = t106 * t902;
    let t11325 = t3882 * t322;
    let t11326 = t3881 * t11325;
    let t11368 = t2718 * t2719 * t116;
    let t11369 = t8384 * t2263;
    (t11025, t11029, t11140, t11325, t11326, t11368, t11369)
}
