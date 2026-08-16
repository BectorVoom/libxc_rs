//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta389 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1420;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1421;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1422;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1423;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1424;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1425;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta389(t41306: f64, t11161: f64, t689: f64, t11150: f64, t39443: f64, t128: f64, t904: f64, t2258: f64, t2853: f64, t2857: f64, t39449: f64, t2850: f64, t41263: f64, t2852: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41307, t41308) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1420(t41306, t11161, t689);
        let (t41310, t41312) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1421(t11150, t39443, t128, t904);
        let (t41314, t41316) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1422(t2258, t2853, t128, t904);
        let (t41318, t41320) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1423(t2857, t39449, t128, t904);
        let t41323 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1424(t128, t2850, t41263);
        let (t41325, t41327) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1425(t2852, t39449, t128, t2850);
    (t41307, t41308, t41310, t41312, t41314, t41316, t41318, t41320, t41323, t41325, t41327)
}
