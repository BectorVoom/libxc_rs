//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 877/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk877<F: Float>(t7624: F, t7695: F, t464: F, t458: F, t1046: F, t2785: F, t1090: F, t2712: F, t1057: F, t2709: F, t2693: F, t7532: F, t7591: F, t1099: F, t2790: F, t7: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7696 = t7624 + t7695;
    let t7697 = t464 * t7696;
    let t7698 = t458 * t7697;
    let t7699 = t1046 * t2785;
    let t7701 = t2712 * t1090;
    let t7703 = t2709 * t1057;
    let t7706 = t7591 * t7532 * t2693;
    let t7708 = 0.10389515463408878255e3 * t1099 * t7706;
    let t7710 = 1.0 / t2790 / t7;
    (t7696, t7697, t7698, t7699, t7701, t7703, t7706, t7708, t7710)
}
