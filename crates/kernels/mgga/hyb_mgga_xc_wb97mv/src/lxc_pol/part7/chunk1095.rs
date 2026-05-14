//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1095/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1095<F: Float>(t11611: F, t491: F, t1089: F, t4509: F, t458: F, t11576: F, t11582: F, t11584: F, t7473: F, t7479: F, t7481: F, t7489: F, t7491: F, t7495: F, t9613: F, t9614: F, t9618: F, t9619: F, t9620: F) -> (F, F, F) {
    let t11612 = t11611 * t491;
    let t11614 = t4509 * t1089;
    let t11615 = t458 * t11614;
    let t11618 = -t11576 + 32.0 * t7473 + 4.0 * t11582 - 4.0 * t11584 + t458 * t11612 + t11615 - t9613 - t7479 + t7481 + 0.48830526149350786811e-3 * t9614 - t9618 - t9619 - t7489 + t7491 + t9620 - 0.5848223622634646207e0 * t7495;
    (t11612, t11614, t11618)
}
