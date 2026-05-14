//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 980/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk980<F: Float>(t2590: F, t3613: F, t1003: F, t260: F, t9402: F, t9404: F, t9415: F, t9443: F, t9446: F, t9448: F, t9451: F, t9457: F, t9518: F, t9557: F, t9573: F, t9577: F, t9579: F, t9581: F, t9584: F, t9587: F, t9590: F, t9594: F, t9597: F, t9598: F) -> (F, F) {
    let t9602 = t3613 * t2590;
    let t9605 = t9402 + t9404 + t260 * (t9457 + t9518 + t9557 + t9598) + t9415 + t9443 + t9446 + t9448 - t9451 + 0.11696447245269292414e1 * t1003 * t9602 + t9573 + t9577 - t9579 + t9581 - t9584 - t9587 - t9590 + t9594 + t9597;
    (t9602, t9605)
}
