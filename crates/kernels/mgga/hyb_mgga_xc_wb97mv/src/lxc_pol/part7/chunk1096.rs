//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1096/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1096<F: Float>(t4509: F, t479: F, t1101: F, t11240: F, t7527: F, t7528: F, t7538: F, t7544: F, t7580: F, t7589: F, t7698: F, t7699: F, t7703: F, t7708: F, t9627: F, t9630: F, t9632: F, t9635: F, t9637: F) -> (F, F) {
    let t11623 = t4509 * t479;
    let t11624 = t11623 * t1101;
    let t11627 = -t7527 - 0.17315859105681463759e2 * t7528 - t7538 + t7544 + t7698 - 8.0 * t7699 - t9627 + 20.0 * t7703 + 0.21687162600603479684e-1 * t9630 + t7708 - 0.5848223622634646207e0 * t11624 - 16.0 * t9632 - t9635 - t9637 - t11240 + t7580 + t7589;
    (t11623, t11627)
}
