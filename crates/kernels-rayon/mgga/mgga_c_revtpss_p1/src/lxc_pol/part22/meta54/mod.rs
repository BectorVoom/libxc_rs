//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta54 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk388;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk389;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk390;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk391;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk392;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk393;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk394;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk395;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk396;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta54(t362: f64, t39: f64, t40: f64, t361: f64, t351: f64, t127: f64, t371: f64, t373: f64, t367: f64, t365: f64, t369: f64, t270: f64, t283: f64, t66: f64, t906: f64, t247: f64, t1003: f64, t1009: f64, t1011: f64, t1017: f64, t1021: f64, t1025: f64, t1028: f64, t1041: f64, t1047: f64, t348: f64, t375: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1052 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk388(t362, t39, t40);
        let t1053 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk389(t1052, t361);
        let (t1054, t1058) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk390(t1053, t351, t127, t371, t373);
        let (t1060, t1062) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk391(t1058, t367, t365, t369, t361);
        let t1063 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk392(t1062, t351);
        let t1065 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk393(t270, t283);
        let t1066 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk394(t1065, t66);
        let t1068 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk395(t1066, t906, t247);
        let t1071 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk396(t1003, t1009, t1011, t1017, t1021, t1025, t1028, t1041, t1047, t1054, t1060, t1063, t1068, t348, t375);
    (t1052, t1053, t1054, t1058, t1060, t1062, t1063, t1065, t1066, t1068, t1071)
}
