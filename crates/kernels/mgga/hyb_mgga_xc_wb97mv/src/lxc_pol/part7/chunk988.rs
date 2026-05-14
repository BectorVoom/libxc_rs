//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 988/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk988<F: Float>(t491: F, t9685: F, t1508: F, t2709: F, t2712: F, t2715: F, t1507: F, t2775: F, t458: F, t3658: F, t479: F, t1101: F, t2685: F, t3638: F, t489: F, t7690: F, t7694: F, t7780: F, t7783: F, t7784: F, t7786: F, t7793: F, t9651: F, t9655: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9686 = t9685 * t491;
    let t9688 = t2709 * t1508;
    let t9690 = t2712 * t1508;
    let t9692 = t2715 * t1508;
    let t9694 = t1507 * t2775;
    let t9695 = t458 * t9694;
    let t9696 = t3658 * t479;
    let t9698 = 0.11696447245269292414e1 * t9696 * t1101;
    let t9699 = t3638 * t2685;
    let t9705 = -0.17315859105681463759e2 * t9651 + t9655 + t458 * t9686 + 20.0 * t9688 + 12.0 * t9690 - 32.0 * t9692 + t9695 + t7780 - t9698 - 0.5848223622634646207e0 * t9699 - t7783 + t7690 + 0.19751673498613801407e-1 * t9685 * t489 + t7694 - 4.0 * t7784 - 4.0 * t7786 - t7793;
    (t9686, t9688, t9690, t9694, t9695, t9696, t9698, t9699, t9705)
}
