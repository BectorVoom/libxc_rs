//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta576 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1762;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1763;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1764;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1765;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1766;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1767;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1768;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta576<F: Float>(t44039: F, t44040: F, t89824: F, t89832: F, t90402: F, t90405: F, t90408: F, t90411: F, t90414: F, t90417: F, t90420: F, t90423: F, t90451: F, t90453: F, t81230: F, t81232: F, t81234: F, t81425: F, t81427: F, t81429: F, t89828: F, t89843: F, t89847: F, t89855: F, t90459: F, t90464: F, t90470: F, t90473: F, t56236: F, t58153: F, t68399: F, t68583: F, t68585: F, t68590: F, t81236: F, t81491: F, t81496: F, t81539: F, t90486: F, t90488: F, t90490: F, t90492: F, t1131: F, t1150: F, t90529: F, t6439: F, t68792: F, t24262: F, t58342: F, t12227: F, t3435: F, t90324: F, t1196: F, t20472: F, t20671: F, t1188: F, t3495: F, t90352: F, t24498: F, t5192: F, t5184: F, t81310: F, t20400: F, t6548: F, t90509: F, t90511: F, t90514: F, t24765: F, t68255: F, t81156: F, t81158: F, t89839: F, t89851: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t90542 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1762::<F>(t44039, t44040, t89824, t89832, t90402, t90405, t90408, t90411, t90414, t90417, t90420, t90423, t90451, t90453);
        let t90558 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1763::<F>(t81230, t81232, t81234, t81425, t81427, t81429, t89828, t89843, t89847, t89855, t90459, t90464, t90470, t90473);
        let t90573 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1764::<F>(t56236, t58153, t68399, t68583, t68585, t68590, t81236, t81491, t81496, t81539, t90486, t90488, t90490, t90492);
        let (t90578, t90580, t90582, t90585) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1765::<F>(t1131, t1150, t90529, t90542, t90558, t90573, t6439, t68792, t24262, t58342, t12227, t3435, t90324);
        let (t90588, t90592, t90594, t90597, t90599) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1766::<F>(t1196, t20472, t20671, t1188, t3495, t90352, t24498, t5192, t5184, t81310, t20400, t6548);
        let t90600 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1767::<F>(t90509, t90511, t90514, t90578, t90580, t90582, t90585, t90588, t90592, t90594, t90597, t90599);
        let (t90602, t90614) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1768::<F>(t24765, t5192, t68255, t81156, t81158, t89824, t89828, t89832, t89839, t89843, t89847, t89851, t89855);
    (t90578, t90580, t90582, t90585, t90588, t90592, t90594, t90597, t90599, t90600, t90602, t90614)
}
