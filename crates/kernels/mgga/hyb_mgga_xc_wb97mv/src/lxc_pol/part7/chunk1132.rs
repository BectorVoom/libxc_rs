//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1132/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1132<F: Float>(t22469: F, t241: F, t238: F, t243: F, t6812: F, t804: F, t2224: F, t2231: F, t2235: F, t222: F, t6129: F, t778: F) -> (F, F, F, F, F, F, F) {
    let t22470 = t22469 * t241;
    let t22472 = t238 * t22470 * t243;
    let t22473 = 0.16979925925925925926e1 * t22472;
    let t22475 = t238 * t6812 * t804;
    let t22478 = t238 * t2224 * t2231;
    let t22481 = t238 * t2224 * t2235;
    let t22498 = t222 * t6129 * t778;
    (t22470, t22472, t22473, t22475, t22478, t22481, t22498)
}
