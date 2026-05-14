//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 986/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk986<F: Float>(t7: F, t2694: F, t3638: F, t1089: F, t3658: F, t458: F, t1173: F, t7710: F, t2791: F, t3: F, t1861: F, t1874: F, t1877: F, t224: F, t3641: F, t3644: F, t457: F, t8917: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t9651 = t3638 * t2694;
    let t9653 = t3658 * t1089;
    let t9655 = 2.0 * t458 * t9653;
    let t9656 = t7710 * t1173;
    let t9659 = t2791 * t3;
    let t9669 = piecewise3(t8, 0.0, -8.0 / 27.0 * t9656 * t1861 + 16.0 / 9.0 * t9659 * t8917 + 4.0 / 9.0 * t3641 * t1877 + 8.0 / 3.0 * t224 * t1874 - 8.0 * t3644 * t457);
    (t9651, t9653, t9655, t9656, t9669)
}
