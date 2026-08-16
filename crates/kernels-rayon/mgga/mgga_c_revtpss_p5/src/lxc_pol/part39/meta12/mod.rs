//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta12 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk79;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk80;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk81;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk82;
use chunk4::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk83;
use chunk5::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk84;
use chunk6::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk85;
use chunk7::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk86;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta12(t128: f64, t16: f64, t65: f64, t64: f64, t159: f64, t206: f64, t122: f64, t124: f64, t136: f64, t196: f64, t149: f64, t191: f64, t194: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t209, t211, t212) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk79(t128);
        let t213 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk80(t209, t212);
        let t215 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk81(t16, t65);
        let t216 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk82(t215, t64);
        let (t217, t218, t220) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk83(t159, t216, t206, t122);
        let t221 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk84(t124, t220);
        let (t222, t225) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk85(t136, t218, t221, t196);
        let t227 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk86(t149, t191, t194, t225);
    (t209, t211, t212, t213, t215, t216, t217, t220, t221, t222, t225, t227)
}
