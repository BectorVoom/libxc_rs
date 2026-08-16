//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1165/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1165<F: Float>(t26594: F, t92232: F, t26576: F, t37013: F, t7579: F, t26602: F, t7580: F, t92226: F, t26579: F, t7583: F, t9229: F, t26580: F, t26611: F) -> (F, F, F, F, F, F, F) {
    let t92307 = t26594 * t92232;
    let t92310 = t37013 * t7579 * t26576;
    let t92312 = t26602 * t26576;
    let t92314 = t7580 * t92226;
    let t92316 = t7580 * t92232;
    let t92319 = t9229 * t26579 * t7583;
    let t92321 = t26580 * t26611;
    (t92307, t92310, t92312, t92314, t92316, t92319, t92321)
}
