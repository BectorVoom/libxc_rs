//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta576 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1762;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1763;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1764;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1765;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1766;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1767;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1768;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta576(t44039: f64, t44040: f64, t89824: f64, t89832: f64, t90402: f64, t90405: f64, t90408: f64, t90411: f64, t90414: f64, t90417: f64, t90420: f64, t90423: f64, t90451: f64, t90453: f64, t81230: f64, t81232: f64, t81234: f64, t81425: f64, t81427: f64, t81429: f64, t89828: f64, t89843: f64, t89847: f64, t89855: f64, t90459: f64, t90464: f64, t90470: f64, t90473: f64, t56236: f64, t58153: f64, t68399: f64, t68583: f64, t68585: f64, t68590: f64, t81236: f64, t81491: f64, t81496: f64, t81539: f64, t90486: f64, t90488: f64, t90490: f64, t90492: f64, t1131: f64, t1150: f64, t90529: f64, t6439: f64, t68792: f64, t24262: f64, t58342: f64, t12227: f64, t3435: f64, t90324: f64, t1196: f64, t20472: f64, t20671: f64, t1188: f64, t3495: f64, t90352: f64, t24498: f64, t5192: f64, t5184: f64, t81310: f64, t20400: f64, t6548: f64, t90509: f64, t90511: f64, t90514: f64, t24765: f64, t68255: f64, t81156: f64, t81158: f64, t89839: f64, t89851: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t90542 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1762(t44039, t44040, t89824, t89832, t90402, t90405, t90408, t90411, t90414, t90417, t90420, t90423, t90451, t90453);
        let t90558 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1763(t81230, t81232, t81234, t81425, t81427, t81429, t89828, t89843, t89847, t89855, t90459, t90464, t90470, t90473);
        let t90573 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1764(t56236, t58153, t68399, t68583, t68585, t68590, t81236, t81491, t81496, t81539, t90486, t90488, t90490, t90492);
        let (t90578, t90580, t90582, t90585) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1765(t1131, t1150, t90529, t90542, t90558, t90573, t6439, t68792, t24262, t58342, t12227, t3435, t90324);
        let (t90588, t90592, t90594, t90597, t90599) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1766(t1196, t20472, t20671, t1188, t3495, t90352, t24498, t5192, t5184, t81310, t20400, t6548);
        let t90600 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1767(t90509, t90511, t90514, t90578, t90580, t90582, t90585, t90588, t90592, t90594, t90597, t90599);
        let (t90602, t90614) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1768(t24765, t5192, t68255, t81156, t81158, t89824, t89828, t89832, t89839, t89843, t89847, t89851, t89855);
    (t90578, t90580, t90582, t90585, t90588, t90592, t90594, t90597, t90599, t90600, t90602, t90614)
}
