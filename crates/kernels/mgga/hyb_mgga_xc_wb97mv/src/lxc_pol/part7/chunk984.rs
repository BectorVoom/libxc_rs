//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 984/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk984<F: Float>(t2627: F, t9629: F, t1046: F, t3668: F, t1041: F, t3666: F, t7580: F, t7589: F, t7610: F, t7639: F, t7698: F, t7699: F, t7703: F, t7708: F, t7738: F, t7742: F, t9610: F, t9627: F) -> (F, F, F, F, F) {
    let t9630 = t9629 * t2627;
    let t9632 = t1046 * t3668;
    let t9635 = 8.0 * t1041 * t3666;
    let t9637 = 8.0 * t1046 * t3666;
    let t9638 = t1041 * t3668;
    let t9641 = t7698 - 16.0 * t7699 - t9627 + 40.0 * t7703 + 0.10843581300301739842e-1 * t9630 + t7708 - 8.0 * t9632 + t9635 - t9637 + 8.0 * t9638 - t9610 + t7580 + t7589 - t7610 + t7639 + 0.21687162600603479684e-1 * t7738 - t7742;
    (t9630, t9632, t9635, t9637, t9641)
}
