//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta212 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1341;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1342;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1343;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1344;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1345;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1346;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta212(t1149: f64, t5108: f64, t3433: f64, t3358: f64, t3439: f64, t5044: f64, t5049: f64, t5054: f64, t5058: f64, t1160: f64, t1737: f64, t1168: f64, t1745: f64, t3415: f64, t3459: f64, t3466: f64, t5072: f64, t5080: f64, t5088: f64, t5090: f64, t5093: f64, t5096: f64, t5099: f64, t5102: f64, t1169: f64, t1744: f64, t3479: f64, t3483: f64, t448: f64, t1179: f64, t1749: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5109, t5111, t5117, t5120) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1341(t1149, t5108, t3433, t3358, t3439, t5044, t5049, t5054, t5058, t1160, t1737);
        let (t5125, t5142) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1342(t1168, t1745, t3358, t3415, t3459, t3466, t5044, t5049, t5054, t5058, t5072, t5080, t5088, t5090, t5093, t5096, t5099, t5102);
        let t5143 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1343(t1169, t5142);
        let t5146 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1344(t1744, t3479);
        let (t5147, t5155) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1345(t1168, t5146, t3358, t3483, t5044, t5049, t5054, t5058);
        let (t5156, t5158) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1346(t448, t5155, t1179, t1749);
    (t5109, t5111, t5117, t5120, t5125, t5142, t5143, t5146, t5147, t5155, t5156, t5158)
}
