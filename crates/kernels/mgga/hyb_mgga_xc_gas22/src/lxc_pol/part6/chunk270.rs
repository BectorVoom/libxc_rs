//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 270/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk270<F: Float>(t899: F, t902: F, t120: F, t307: F, t328: F, t332: F, t319: F, t97: F, t99: F, t315: F, t324: F, t122: F, t331: F, tau0: F) -> (F, F, F, F, F, F, F) {
    let t903 = t899 * t902;
    let t907 = t328 * t307 * t120;
    let t908 = t907 * t332;
    let t909 = t319 * t97;
    let t911 = F::new(1.0) / t99 / t909;
    let t912 = t315 * t911;
    let t913 = t324 * tau0;
    let t914 = t912 * t913;
    let t918 = F::new(1.0) / t331 / t122;
    (t903, t907, t908, t909, t913, t914, t918)
}
