//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta160 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1016;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1017;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1018;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1019;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1020;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1021;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1022;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1023;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1024;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta160(t225: f64, t3727: f64, t494: f64, t1269: f64, t460: f64, t1275: f64, t493: f64, t1294: f64, t1204: f64, t1284: f64, t1280: f64, t3568: f64, t487: f64, t1209: f64, t1287: f64, t3721: f64, t473: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3729, t3732) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1016(t225, t3727, t494, t1269, t460);
        let (t3736, t3737) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1017(t1275, t493, t225);
        let t3738 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1018(t1294);
        let t3739 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1019(t3737, t3738);
        let t3746 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1020(t1204, t1284);
        let (t3751, t3754) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1021(t1280, t3568, t1284, t487);
        let t3755 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1022(t1209, t3754);
        let t3756 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1023(t1287, t3721);
        let t3759 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1024(t1269, t473);
    (t3729, t3732, t3736, t3737, t3738, t3739, t3746, t3751, t3754, t3755, t3756, t3759)
}
