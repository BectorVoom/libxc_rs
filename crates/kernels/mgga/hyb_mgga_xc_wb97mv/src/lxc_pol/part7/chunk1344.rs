//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1344/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1344<F: Float>(t11922: F, t1291: F, t516: F, t1298: F, t1114: F, t11809: F, t3746: F, t11889: F, t7899: F, t11901: F, t9831: F, t1148: F, t3704: F, t27911: F, t3736: F, t1801: F, t297: F, t4077: F, t9913: F) -> (F, F, F, F, F, F, F, F, F) {
    let t32834 = t516 * t11922 * t1291;
    let t32838 = t516 * t11922 * t1298;
    let t32842 = t3746 * t11809 * t1114;
    let t32845 = t11889 * t7899;
    let t32848 = t11901 * t9831;
    let t32851 = t11901 * t7899;
    let t32861 = t1148 * t3704;
    let t32870 = t3736 * t27911;
    let t32875 = t9913 * t4077 * t297 * t1801;
    (t32834, t32838, t32842, t32845, t32848, t32851, t32861, t32870, t32875)
}
