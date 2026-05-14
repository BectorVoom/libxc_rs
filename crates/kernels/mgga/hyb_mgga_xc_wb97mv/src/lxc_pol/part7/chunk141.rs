//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 141/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk141<F: Float>(t403: F, t408: F, t209: F, t313: F, t316: F, t211: F, t322: F, t396: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t409 = t403 * t408;
    let t412 = t313 * t209;
    let t414 = t209 * t209;
    let t415 = t316 * t414;
    let t416 = t211 * t211;
    let t417 = 1.0 / t416;
    let t419 = t414 * t414;
    let t420 = t322 * t419;
    let t421 = t416 * t416;
    let t422 = 1.0 / t421;
    let t423 = t420 * t422;
    let t424 = t396 * sigma2;
    (t409, t412, t414, t415, t416, t417, t420, t421, t422, t423, t424)
}
