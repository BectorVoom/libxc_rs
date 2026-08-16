//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta8 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk54;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk55;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk56;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk57;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk58;
use chunk5::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk59;
use chunk6::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk60;
use chunk7::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk61;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta8(t125: f64, t126: f64, t123: f64, t72: f64, t122: f64, t66: f64, t124: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t127 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk54(t125, t126);
        let t128 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk55(t123, t127);
        let t130 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk56(t128);
        let t131 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk57(t128);
        let (t134, t136) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk58(t128, t72);
        let (t137, t138) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk59(t122, t136);
        let (t139, t140) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk60(t66, t124);
        let t141 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk61(t138, t140);
    (t127, t128, t130, t131, t134, t136, t137, t138, t139, t140, t141)
}
