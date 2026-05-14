//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 227/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk227<F: Float>(t143: F, t674: F, t698: F, t701: F, t571: F, t695: F) -> (F, F, F) {
    let t145 = 0.135e1 < t143;
    let t703 = t698 * t701 * t674;
    let t706 = -t571 * t703 / 54.0 - t695 / 54.0;
    let t707 = piecewise3(t145, t706, 0.0);
    (t703, t706, t707)
}
