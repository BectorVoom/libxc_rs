//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta2 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk17;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk18;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk19;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk20;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta2(t37: f64, rho0: f64, sigma0: f64, t36: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t38 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk17(t37);
        let (t39, t40, t41, t43, t44) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk18(rho0, sigma0);
        let t45 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk19(t36);
        let (t46, t47, t48) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk20(t45);
    (t38, t39, t40, t41, t43, t44, t45, t46, t47, t48)
}
