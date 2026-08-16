//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta9 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk62;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk63;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk64;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk65;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk66;
use chunk5::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk67;
use chunk6::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk68;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta9(t128: f64, t131: f64, t134: f64, t141: f64, t130: f64, t37: f64, t45: f64, zeta_threshold: f64, t79: f64, t57: f64, t82: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t143, t146, t147, t149) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk62(t128, t131, t134, t141, t130);
        let t150 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk63(t37);
        let (t152, t153) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk64(t45, zeta_threshold);
        let t157 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk65(t45, t153, t79, t57, t82, zeta_threshold);
        let (t158, t159) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk66(t150, t157);
        let t162 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk67(t159);
        let t164 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk68(t128);
    (t143, t146, t147, t149, t150, t152, t153, t157, t158, t159, t162, t164)
}
