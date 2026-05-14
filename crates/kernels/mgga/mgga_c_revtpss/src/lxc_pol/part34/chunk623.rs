//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 623/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk623<F: Float>(t1715: F, t5277: F, t1042: F, t6435: F, t6437: F, t6441: F, t6473: F, t6476: F, t6542: F, t6544: F, t6546: F, t6550: F, t6554: F, t6558: F) -> (F, F, F) {
    let t6618 = t5277 * t1715;
    let t6619 = t1042 * t6618;
    let t6622 = -t6435 + t6437 - t6441 + t6473 + t6476 + t6542 + t6544 - t6546 + t6550 - t6554 - t6558;
    (t6618, t6619, t6622)
}
