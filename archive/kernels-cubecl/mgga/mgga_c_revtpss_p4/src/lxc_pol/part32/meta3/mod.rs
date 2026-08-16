//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta3 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk21;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk22;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk23;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk24;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta3<F: Float>(t46: F, t48: F, rho1: F, sigma2: F, t36: F, sigma0: F, sigma1: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t49, t51, t52, t53, t55, t56) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk21::<F>(t46, t48, rho1, sigma2);
        let t57 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk22::<F>(t36);
        let (t58, t59, t60) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk23::<F>(t57);
        let (t61, t64) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk24::<F>(t58, t60, sigma0, sigma1, sigma2);
    (t49, t51, t52, t53, t55, t56, t57, t58, t59, t60, t61, t64)
}
