//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1098/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1098<F: Float>(t6466: F, t677: F, t1815: F, t2026: F, t2024: F, t2029: F, t2021: F, t2053: F, t10: F, t138: F, t2054: F, t6291: F, t2034: F, t6469: F, t684: F, t3150: F, t689: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20171 = t677 * t6466;
    let t20216 = t1815 * t2026;
    let t20218 = t2024 * t20216 * t2029;
    let t20225 = 1.0 / t2053 / t2021;
    let t20226 = t20225 * t10;
    let t20229 = 1.0 / t138 / t2054;
    let t20241 = t6291 * t10;
    let t20252 = t684 * t6469 * t2034;
    let t20255 = t684 * t3150 * t689;
    (t20171, t20216, t20218, t20225, t20226, t20229, t20241, t20252, t20255)
}
