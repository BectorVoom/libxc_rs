//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta4 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk33;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk34;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk35;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk36;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk37;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk38;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk39;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk40;
use chunk8::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk41;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta4(t16: f64, t66: f64, t64: f64, t44: f64, t49: f64, t56: f64, t61: f64, t38: f64, t45: f64, t57: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t68 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk33(t16, t66);
        let t69 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk34(t64, t68);
        let t70 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk35(t44, t49, t56, t61, t69);
        let t71 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk36(t38, t70);
        let t72 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk37();
        let t73 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk38();
        let (t76, t77) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk39(t73, t72);
        let t78 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk40(t45);
        let (t79, t80, t81) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk41(t45, t78, t57);
    (t68, t69, t70, t71, t72, t73, t76, t77, t78, t79, t80, t81)
}
