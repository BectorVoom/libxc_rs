//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta63 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk454;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk455;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk456;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk457;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk458;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk459;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk460;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk461;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta63(t1230: f64, t480: f64, t1209: f64, t225: f64, t1214: f64, t482: f64, t372: f64, t371: f64, t1032: f64, t460: f64, t472: f64, t474: f64, t1038: f64, t479: f64, t1128: f64, t1153: f64, t1193: f64, t1195: f64, t1200: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1231, t1234) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk454(t1230, t480, t1209, t225);
        let t1235 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk455(t1234, t480);
        let (t1236, t1238) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk456(t1214, t482, t372, t371);
        let (t1241, t1242, t1243) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk457(t1032, t460, t472);
        let t1244 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk458(t1243, t474);
        let t1246 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk459(t1038, t479, t1244);
        let t1247 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk460(t1241, t1246);
        let t1248 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk461(t1128, t1153, t1193, t1195, t1200);
    (t1231, t1234, t1235, t1236, t1238, t1241, t1242, t1243, t1244, t1246, t1247, t1248)
}
