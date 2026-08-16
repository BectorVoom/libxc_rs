//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta217 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1367;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1368;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1369;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1370;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1371;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1372;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1373;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1374;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1375;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta217(t1032: f64, t1770: f64, t1246: f64, t1263: f64, t1774: f64, t1122: f64, t1042: f64, t5062: f64, t5065: f64, t5067: f64, t5070: f64, t5107: f64, t5111: f64, t5189: f64, t5191: f64, t5194: f64, t5196: f64, t5200: f64, t5204: f64, t5209: f64, t1250: f64, t482: f64, t1038: f64, t1802: f64, t1244: f64, t1241: f64, t1121: f64, t1214: f64, t1469: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5273, t5274) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1367(t1032, t1770, t1246);
        let t5277 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1368(t1263, t1774);
        let (t5278, t5279) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1369(t1122, t5277, t1042);
        let t5284 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1370(t5062, t5065, t5067, t5070, t5107, t5111, t5189, t5191, t5194, t5196, t5200, t5204, t5209);
        let (t5286, t5287) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1371(t1250, t482, t5284, t1042);
        let t5292 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1372(t1038, t1802, t1244);
        let t5293 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1373(t1241, t5292);
        let t5296 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1374(t1121, t1263);
        let t5297 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1375(t1214, t1469);
    (t5273, t5274, t5277, t5278, t5279, t5284, t5286, t5287, t5292, t5293, t5296, t5297)
}
