//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta4 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk25;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk26;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk27;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk28;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk29;
use chunk5::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk30;
use chunk6::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk31;
use chunk7::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk32;
use chunk8::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk33;
use chunk9::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk34;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta4(t3: f64, t16: f64, t64: f64, t44: f64, t49: f64, t56: f64, t61: f64, t38: f64, t45: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t65 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk25(t3);
        let t66 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk26(t65);
        let t68 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk27(t16, t66);
        let t69 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk28(t64, t68);
        let t70 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk29(t44, t49, t56, t61, t69);
        let t71 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk30(t38, t70);
        let t72 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk31();
        let t73 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk32();
        let t77 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk33(t73, t72);
        let t78 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk34(t45);
    (t65, t66, t68, t69, t70, t71, t72, t73, t77, t78)
}
