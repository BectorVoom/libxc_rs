//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1130/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1130<F: Float>(t2197: F, t2243: F, t230: F, t2191: F, t2198: F, t6918: F, t782: F, t2299: F, t2321: F, t275: F, t2293: F, t2300: F, t2254: F, t2283: F, t6902: F, t815: F) -> (F, F, F, F, F, F, F, F) {
    let t22399 = t230 / t2243 / t2197;
    let t22404 = t2191 * t2198;
    let t22407 = t782 * t6918;
    let t22427 = 1.0 / t2321 / t2299;
    let t22428 = t275 * t22427;
    let t22432 = t2293 * t2300;
    let t22435 = t2254 * t2283;
    let t22440 = t815 * t6902;
    (t22399, t22404, t22407, t22427, t22428, t22432, t22435, t22440)
}
