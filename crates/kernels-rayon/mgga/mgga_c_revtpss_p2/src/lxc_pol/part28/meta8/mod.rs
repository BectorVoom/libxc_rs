//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta8 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk57;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk58;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk59;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk60;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk61;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk62;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk63;
use chunk7::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk64;
use chunk8::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk65;
use chunk9::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk66;
use chunk10::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk67;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta8(t65: f64, t125: f64, t123: f64, t72: f64, t122: f64, t66: f64, t124: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t126 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk57(t65);
        let t127 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk58(t125, t126);
        let t128 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk59(t123, t127);
        let t130 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk60(t128);
        let t131 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk61(t128);
        let (t134, t136) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk62(t128, t72);
        let t137 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk63(t122);
        let t138 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk64(t136, t137);
        let t139 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk65(t66);
        let t140 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk66(t124, t139);
        let t141 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk67(t138, t140);
    (t126, t127, t128, t130, t131, t134, t136, t137, t138, t139, t140, t141)
}
