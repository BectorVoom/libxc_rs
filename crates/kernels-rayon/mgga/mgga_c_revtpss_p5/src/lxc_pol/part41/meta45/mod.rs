//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta45 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk280;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk281;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk282;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk283;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk284;
use chunk5::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk285;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta45(t198: f64, t207: f64, t679: f64, t704: f64, t709: f64, t718: f64, t751: f64, t754: f64, t759: f64, t764: f64, t765: f64, t775: f64, t890: f64, t892: f64, t159: f64, t675: f64, t268: f64, t271: f64, t373: f64, t631: f64, t606: f64, t128: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t895 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk280(t198, t207, t679, t704, t709, t718, t751, t754, t759, t764, t765, t775, t890, t892);
        let (t900, t902) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk281(t159, t675, t268, t271);
        let (t903, t904) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk282(t902, t159, t373);
        let t905 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk283(t631);
        let t906 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk284(t606, t905);
        let (t907, t908) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk285(t904, t906, t128);
    (t895, t900, t902, t903, t904, t905, t906, t907, t908)
}
