//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1134/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1134<F: Float>(t521: F, t536: F, t509: F, t523: F, t2880: F, t2938: F, t526: F, t7572: F, t527: F, t2856: F, t528: F, t530: F, t2867: F, t7805: F, t1143: F, t9557: F) -> (F, F, F, F, F, F, F, F) {
    let t22645 = t536 * t521;
    let t22652 = t523 * t509;
    let t22653 = t22652 * t521;
    let t22662 = t2938 * t2880;
    let t22703 = t7572 * t526;
    let t22705 = 1.0 / t527 / t22703;
    let t22714 = 1.0 / t530 / t2856 / t528 / 2.0;
    let t22723 = 1.0 / t22703;
    let t22746 = t2867 * t7805;
    let t22750 = t1143 * t9557;
    (t22645, t22653, t22662, t22705, t22714, t22723, t22746, t22750)
}
