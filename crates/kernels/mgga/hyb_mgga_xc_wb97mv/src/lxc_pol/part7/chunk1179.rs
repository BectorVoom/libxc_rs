//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1179/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1179<F: Float>(t762: F, t8535: F, t3155: F, t3159: F, t8542: F, t8523: F, t8543: F, t13: F, t21924: F, t2986: F, t191: F, t240: F, t6736: F, t8528: F, t8531: F, t8548: F) -> (F, F, F, F, F, F, F) {
    let t26205 = t8535 * t762;
    let t26211 = t3155 * t8542 * t762 * t3159;
    let t26217 = t3155 * t8543 * t8523;
    let t26220 = t21924 * t13 * t2986;
    let t26226 = t240 * t6736 * t191;
    let t26228 = t8528 * t26226 * t8531;
    let t26234 = t8548 * t8543 * t8531;
    (t26205, t26211, t26217, t26220, t26226, t26228, t26234)
}
