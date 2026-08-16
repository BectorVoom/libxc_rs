//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta3 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk22;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk23;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk24;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk25;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta3(t46: f64, t48: f64, rho1: f64, sigma2: f64, t36: f64, sigma0: f64, sigma1: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49, t51, t52, t53, t55, t56) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk22(t46, t48, rho1, sigma2);
        let t57 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk23(t36);
        let (t58, t59, t60) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk24(t57);
        let (t61, t64) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk25(t58, t60, sigma0, sigma1, sigma2);
    (t49, t51, t52, t53, t55, t56, t57, t58, t59, t60, t61, t64)
}
