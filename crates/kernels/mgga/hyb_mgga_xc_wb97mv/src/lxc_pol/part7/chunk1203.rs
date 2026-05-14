//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1203/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1203<F: Float>(t24272: F, t535: F, t10155: F, t2952: F, t10140: F, t1126: F, t2831: F, t7926: F, t9838: F, t7907: F, t9872: F, t7817: F, t1142: F, t10189: F, t1157: F, t10176: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t28585 = t535 * t24272;
    let t28617 = t2952 * t10155;
    let t28621 = t1126 * t10140;
    let t28634 = t7926 * t2831;
    let t28638 = t7926 * t9838;
    let t28644 = t535 * t7907 * sigma0;
    let t28648 = t2952 * t9872;
    let t28666 = t2952 * t7817;
    let t28676 = t7817 * t1142;
    let t28677 = t535 * t28676;
    let t28682 = t1157 * t10189;
    let t28686 = t1126 * t10176;
    (t28585, t28617, t28621, t28634, t28638, t28644, t28648, t28666, t28676, t28677, t28682, t28686)
}
