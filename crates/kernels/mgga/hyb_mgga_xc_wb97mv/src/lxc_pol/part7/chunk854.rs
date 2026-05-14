//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 854/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk854<F: Float>(t346: F, t353: F, t7189: F, t343: F, t238: F, t351: F, t6812: F, t2224: F, t952: F) -> (F, F, F, F, F, F, F) {
    let t7266 = 1.0 / t346 / t353 / 4.0;
    let t7273 = 28.0 / 27.0 * t7189;
    let t7278 = 0.93932222222222222223e0 * t7189;
    let t7282 = 1.0/pow_3_2(t343);
    let t7291 = t238 * t6812 * t351;
    let t7292 = 0.36793333333333333333e0 * t7291;
    let t7294 = t238 * t2224 * t952;
    (t7266, t7273, t7278, t7282, t7291, t7292, t7294)
}
