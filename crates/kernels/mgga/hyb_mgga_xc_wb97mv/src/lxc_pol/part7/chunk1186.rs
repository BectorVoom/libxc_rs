//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1186/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1186<F: Float>(t3435: F, t6968: F, t3402: F, t6905: F, t2322: F, t3413: F, t1357: F, t6965: F, t2300: F, t6981: F, t2261: F, t3380: F, t1345: F, t6875: F, t819: F, t9014: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26752 = t3435 * t6968;
    let t26786 = t3402 * t6905;
    let t26834 = t3413 * t2322;
    let t26839 = t1357 * t6965;
    let t26846 = t3413 * t2300;
    let t26850 = t1357 * t6981;
    let t26853 = t3380 * t2261;
    let t26856 = t1345 * t6875;
    let t26924 = t9014 * t819;
    (t26752, t26786, t26834, t26839, t26846, t26850, t26853, t26856, t26924)
}
