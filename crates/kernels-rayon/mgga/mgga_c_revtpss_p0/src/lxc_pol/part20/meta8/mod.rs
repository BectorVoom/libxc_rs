//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta8 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk65;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk66;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk67;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk68;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk69;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk70;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk71;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta8(t128: f64, t72: f64, t122: f64, t66: f64, t124: f64, t131: f64, t130: f64, t37: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t134, t136) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk65(t128, t72);
        let (t137, t138) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk66(t122, t136);
        let (t139, t140) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk67(t66, t124);
        let t141 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk68(t138, t140);
        let (t143, t146, t147) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk69(t128, t131, t134, t141);
        let t149 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk70(t130, t147);
        let t150 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk71(t37);
    (t134, t136, t137, t138, t139, t140, t141, t143, t146, t147, t149, t150)
}
