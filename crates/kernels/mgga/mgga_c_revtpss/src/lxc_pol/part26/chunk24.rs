//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 24/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk24<F: Float>(t57: F, sigma0: F, sigma1: F, sigma2: F) -> (F, F, F, F, F) {
    let t58 = t57 / 2.0;
    let t59 = pow_1_3(t58);
    let t60 = t59 * t59;
    let t61 = t60 * t58;
    let t64 = sigma0 + 2.0 * sigma1 + sigma2;
    (t58, t59, t60, t61, t64)
}
