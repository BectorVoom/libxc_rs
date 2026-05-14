//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 985/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk985<F: Float>(t7758: F, t2704: F, t3638: F, t7645: F, t7649: F, t7653: F, t7657: F, t7745: F, t7748: F, t7751: F, t7752: F, t7757: F, t7760: F, t7767: F, t7772: F, t7774: F, t7777: F) -> (F, F, F) {
    let t9644 = 32.0 * t7758;
    let t9648 = t3638 * t2704;
    let t9650 = t7745 + t7748 - t7751 + 0.23392894490538584828e1 * t7752 - t7757 - t9644 - 24.0 * t7760 + t7767 - t7772 - 0.18311447306006545054e-3 * t7774 - t7645 - t7649 + 0.4883052614935078681e-3 * t7777 + t7653 + t7657 + 0.11696447245269292414e1 * t9648;
    (t9644, t9648, t9650)
}
