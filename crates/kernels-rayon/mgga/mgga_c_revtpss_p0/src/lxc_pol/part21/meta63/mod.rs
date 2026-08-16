//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta63 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk459;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk460;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk461;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk462;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk463;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk464;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk465;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk466;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk467;
use chunk9::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk468;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta63(t1038: f64, t479: f64, t1244: f64, t1241: f64, t1128: f64, t1153: f64, t1193: f64, t1195: f64, t1200: f64, t471: f64, t73: f64, t482: f64, t1042: f64, t127: f64, t371: f64, t481: f64, t369: f64, t475: f64, t467: f64, t403: f64, t414: f64, t66: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1246 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk459(t1038, t479, t1244);
        let t1247 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk460(t1241, t1246);
        let t1248 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk461(t1128, t1153, t1193, t1195, t1200);
        let t1250 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk462(t471, t73);
        let (t1251, t1252) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk463(t1248, t1250, t482, t1042);
        let t1256 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk464(t127, t371, t482);
        let (t1258, t1260) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk465(t1256, t481, t369, t479, t475);
        let t1261 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk466(t1260, t467);
        let t1263 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk467(t403, t414);
        let t1264 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk468(t1263, t66);
    (t1246, t1247, t1248, t1250, t1251, t1252, t1256, t1258, t1260, t1261, t1263, t1264)
}
