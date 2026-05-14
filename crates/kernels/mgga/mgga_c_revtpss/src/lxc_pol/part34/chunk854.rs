//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 854/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk854<F: Float>(t114: F, t22628: F, t655: F, t10201: F, t13448: F, t21818: F, t21827: F, t22590: F, t22593: F, t69: F) -> (F,) {
    let t115 = 1.0 < t114;
    let t22629 = t655 * t22628;
    let t22633 = piecewise3(t115, 0.0, -t10201 - 11.0 / 3.0 * t13448 - 2.0 * t21818 + t21827 - 3.0 / 4.0 * t69 * t22590 + 3.0 / 4.0 * t69 * t22593 - t69 * t22629 / 8.0);
    (t22633,)
}
