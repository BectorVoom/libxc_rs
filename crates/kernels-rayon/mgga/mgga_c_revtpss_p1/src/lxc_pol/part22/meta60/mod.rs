//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta60 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk435;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk436;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk437;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk438;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk439;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk440;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta60(t1118: f64, t1124: f64, t448: f64, t444: f64, t439: f64, t1143: f64, t1135: f64, t1140: f64, t1147: f64, t447: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1173, t1175) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk435(t1118, t1124);
        let (t1176, t1178, t1179) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk436(t1175, t448, t444);
        let t1180 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk437(t1179, t439);
        let (t1182, t1185, t1187) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk438(t1118, t1143, t1124, t1135, t1140, t1147);
        let t1188 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk439(t447);
        let t1189 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk440(t1187, t1188);
    (t1173, t1175, t1176, t1178, t1179, t1180, t1182, t1185, t1187, t1188, t1189)
}
