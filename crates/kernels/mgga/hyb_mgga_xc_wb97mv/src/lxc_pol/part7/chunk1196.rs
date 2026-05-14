//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1196/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1196<F: Float>(t7740: F, t9629: F, t221: F, t2627: F, t3658: F, t1508: F, t7492: F, t2775: F, t458: F, t1507: F, t7696: F, t3661: F, t7791: F, t2712: F, t3668: F, t3666: F) -> (F, F, F, F, F, F, F, F) {
    let t27776 = t9629 * t7740;
    let t27781 = t3658 * t221 * t2627;
    let t27783 = t7492 * t1508;
    let t27786 = t458 * t3658 * t2775;
    let t27789 = t458 * t1507 * t7696;
    let t27792 = t3661 * t7791;
    let t27794 = t2712 * t3668;
    let t27796 = t2712 * t3666;
    (t27776, t27781, t27783, t27786, t27789, t27792, t27794, t27796)
}
