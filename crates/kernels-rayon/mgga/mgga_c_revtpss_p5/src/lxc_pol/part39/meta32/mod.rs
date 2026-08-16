//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta32 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk202;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk203;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk204;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk205;
use chunk4::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk206;
use chunk5::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk207;
use chunk6::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk208;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta32(t48: f64, t606: f64, t60: f64, t579: f64, t66: f64, t64: f64, t44: f64, t49: f64, t56: f64, t614: f64, t38: f64, t45: f64, t78: f64, t57: f64, t81: f64, t77: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t617, t620, t624, t625) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk202(t48, t606, t60, t579, t66, t64);
        let (t626, t627) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk203(t625, t44, t49, t56, t614, t617, t620);
        let (t628, t631) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk204(t38, t627, t45);
        let t633 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk205(t631, t78);
        let t635 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk206(t57);
        let t637 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk207(t635, t81);
        let t641 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk208(t606, t633, t637, t77);
    (t617, t620, t624, t625, t626, t627, t628, t631, t633, t635, t637, t641)
}
