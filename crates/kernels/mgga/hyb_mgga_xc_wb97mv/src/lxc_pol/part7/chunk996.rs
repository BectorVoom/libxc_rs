//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 996/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk996<F: Float>(t1157: F, t9838: F, t522: F, t653: F, t198: F, t3736: F, t9831: F, t1142: F, t2822: F, t516: F, t1514: F, t1114: F, t3747: F, t3746: F, t7899: F, t535: F, t7853: F, sigma0: F, tau0: F, tau1: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9839 = t1157 * t9838;
    let t9840 = t653 * t522;
    let t9841 = tau0 * tau1;
    let t9842 = t9841 * t198;
    let t9843 = t9840 * t9842;
    let t9846 = t3736 * t9831;
    let t9849 = t2822 * t1142;
    let t9850 = t516 * t9849;
    let t9851 = tau0 * t653;
    let t9852 = t1514 * sigma0;
    let t9853 = t9851 * t9852;
    let t9856 = t3747 * t1114;
    let t9857 = t3746 * t9856;
    let t9862 = t3736 * t7899;
    let t9865 = t535 * t7853;
    (t9839, t9840, t9841, t9842, t9843, t9846, t9849, t9850, t9851, t9852, t9853, t9856, t9857, t9862, t9865)
}
