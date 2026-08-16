//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta40 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk276;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk277;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk278;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk279;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk280;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk281;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk282;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta40(t124: f64, t775: f64, t800: f64, t212: f64, t27: f64, t235: f64, t240: f64, t234: f64, t243: f64, t236: f64, t786: f64, t213: f64, t225: f64, t232: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t802, t807) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk276(t124, t775, t800, t212, t27);
        let t808 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk277(t235, t240);
        let t810 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk278(t234, t243, t808);
        let (t812, t813, t814) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk279(t807, t810, t236, t786, t240, t27);
        let (t815, t816) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk280(t243, t814, t124, t800);
        let (t819, t820) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk281(t815, t816, t813, t213, t225);
        let (t821, t822) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk282(t232);
    (t802, t807, t808, t810, t812, t813, t814, t816, t819, t820, t821, t822)
}
