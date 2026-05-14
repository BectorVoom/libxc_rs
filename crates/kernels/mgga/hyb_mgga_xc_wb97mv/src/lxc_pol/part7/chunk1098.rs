//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1098/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1098<F: Float>(t10: F, t4509: F, t1096: F, t11611: F, t489: F, t7653: F, t7657: F, t7690: F, t7694: F, t7783: F, t7788: F, t7793: F, t9648: F, t9651: F, t9655: F, t9688: F, t9690: F, t9695: F, t9698: F, t9699: F) -> (F, F) {
    let t11641 = t4509 * t10;
    let t11642 = t11641 * t1096;
    let t11646 = t7653 + t7657 + 0.23392894490538584828e1 * t9648 - 0.34631718211362927517e2 * t9651 + t9655 + 40.0 * t9688 - 24.0 * t9690 + 2.0 * t9695 - t9698 - 0.11696447245269292414e1 * t9699 - t7783 + t7690 + t7694 - 8.0 * t7788 - 0.18311447306006545054e-3 * t11642 + 0.19751673498613801407e-1 * t11611 * t489 - t7793;
    (t11641, t11646)
}
