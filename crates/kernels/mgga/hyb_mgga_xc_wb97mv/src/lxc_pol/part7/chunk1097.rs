//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1097/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1097<F: Float>(t7610: F, t7639: F, t7645: F, t7649: F, t7738: F, t7742: F, t7745: F, t7748: F, t7751: F, t7752: F, t7757: F, t7760: F, t7767: F, t7772: F, t7777: F, t9644: F) -> (F,) {
    let t11633 = -t7610 + t7639 + 0.10843581300301739842e-1 * t7738 - t7742 + t7745 + t7748 - t7751 + 0.11696447245269292414e1 * t7752 - t7757 + t9644 + 12.0 * t7760 + t7767 - t7772 - t7645 - t7649 + 0.24415263074675393405e-3 * t7777;
    (t11633,)
}
