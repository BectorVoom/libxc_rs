//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 740/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk740<F: Float>(t3877: F, t54: F, t3894: F, t587: F, t57: F, t591: F, t60: F, t595: F, t63: F, t599: F, t66: F, t603: F, t69: F, t607: F, t1911: F, t611: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3897 = t54 * t3877;
    let t3899 = t587 * t3894;
    let t3901 = t57 * t3877;
    let t3903 = t591 * t3894;
    let t3905 = t60 * t3877;
    let t3907 = t595 * t3894;
    let t3909 = t63 * t3877;
    let t3911 = t599 * t3894;
    let t3913 = t66 * t3877;
    let t3915 = t603 * t3894;
    let t3917 = t69 * t3877;
    let t3919 = t607 * t3894;
    let t3921 = t1911 * t3877;
    let t3923 = t611 * t3894;
    (t3897, t3899, t3901, t3903, t3905, t3907, t3909, t3911, t3913, t3915, t3917, t3919, t3921, t3923)
}
