//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 683/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk683<F: Float>(t3494: F, t958: F, t1404: F, t2469: F, t956: F, t2474: F, t1393: F, t2480: F, t941: F, t2453: F, t2484: F, t3478: F, t3489: F) -> (F, F, F, F, F, F, F) {
    let t3496 = 1.0 * t3494 * t958;
    let t3498 = 1.0 * t2469 * t1404;
    let t3499 = t1404 * t956;
    let t3501 = 2.0 * t2474 * t3499;
    let t3502 = t2480 * t1393;
    let t3503 = t3502 * t941;
    let t3507 = t2484 - t2453 / 3.0 - t3478 / 3.0 + t3489;
    (t3496, t3498, t3499, t3501, t3502, t3503, t3507)
}
