//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta5 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk42;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk43;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk44;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk45;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk46;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk47;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta5(t57: f64, t81: f64, t80: f64, t77: f64, t71: f64, t5: f64, t10: f64, t11: f64, t12: f64, t29: f64, t9: f64, t41: f64, rho0: f64, tau0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t82, t83, t84, t85) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk42(t57, t81, t80, t77);
        let (t88, t89, t90) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk43(t71, t85);
        let t91 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk44(t90);
        let t93 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk45(t5, t10, t11, t12, t29, t9, t91);
        let t94 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk46(t93);
        let t97 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk47(t41, rho0, tau0);
    (t82, t83, t84, t85, t88, t89, t90, t91, t93, t94, t97)
}
