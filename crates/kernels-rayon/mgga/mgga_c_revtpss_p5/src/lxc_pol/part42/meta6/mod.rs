//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta6 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk39;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk40;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk41;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk42;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk43;
use chunk5::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk44;
use chunk6::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk45;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta6(t93: f64, t41: f64, rho0: f64, tau0: f64, t30: f64, t53: f64, rho1: f64, tau1: f64, t33: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t94 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk39(t93);
        let t97 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk40(t41, rho0, tau0);
        let (t98, t99, t100) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk41(t30);
        let (t101, t104) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk42(t100, t98, t53, rho1);
        let t105 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk43(t104, tau1);
        let (t106, t107, t108) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk44(t33);
        let t109 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk45(t106, t108);
    (t94, t97, t98, t99, t100, t101, t104, t105, t106, t107, t108, t109)
}
