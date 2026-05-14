//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1204/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1204<F: Float>(t1157: F, t11896: F, t1126: F, t11922: F, t2822: F, t3728: F, t297: F, t513: F, t1142: F, t7837: F, t516: F, t1153: F, t7938: F, t16106: F, t522: F, t28676: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t28701 = t1157 * t11896;
    let t28705 = t1126 * t11922;
    let t28729 = t3728 * t2822;
    let t28748 = t513 * t297;
    let t28749 = t1126 * t28748;
    let t28754 = t7837 * t1142;
    let t28755 = t516 * t28754;
    let t28787 = t7938 * t1153;
    let t28833 = t1126 * t28754;
    let t28838 = t516 * t16106 * t522;
    let t28844 = t1157 * t28676;
    (t28701, t28705, t28729, t28748, t28749, t28755, t28787, t28833, t28838, t28844)
}
