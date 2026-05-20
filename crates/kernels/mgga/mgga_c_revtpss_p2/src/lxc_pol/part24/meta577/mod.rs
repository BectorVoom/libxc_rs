//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta577 (260520-c91 hierarchical CSE).
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
mod chunk10;
mod chunk11;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1769;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1770;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1771;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1772;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1773;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1774;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1775;
use chunk7::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1776;
use chunk8::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1777;
use chunk9::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1778;
use chunk10::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1779;
use chunk11::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1780;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta577<F: Float>(t45000: F, t56236: F, t68257: F, t68399: F, t81230: F, t81232: F, t81234: F, t81236: F, t89865: F, t89869: F, t89873: F, t89877: F, t422: F, t90614: F, t20400: F, t6556: F, t1196: F, t24408: F, t5197: F, t24473: F, t5192: F, t1188: F, t12485: F, t90357: F, t12555: F, t43752: F, t6486: F, t68255: F, t81156: F, t81158: F, t89839: F, t89851: F, t90379: F, t90384: F, t90387: F, t90390: F, t45106: F, t45107: F, t89824: F, t89832: F, t90402: F, t90405: F, t90408: F, t90411: F, t90414: F, t90417: F, t90420: F, t90423: F, t90451: F, t90453: F, t81425: F, t81427: F, t81429: F, t89828: F, t89843: F, t89847: F, t89855: F, t90459: F, t90464: F, t90470: F, t90473: F, t58153: F, t68583: F, t68585: F, t68590: F, t81491: F, t81496: F, t81539: F, t90486: F, t90488: F, t90490: F, t90492: F, t1161: F, t1169: F, t1180: F, t12472: F, t1745: F, t1757: F, t20526: F, t20542: F, t24363: F, t24366: F, t24411: F, t45085: F, t45157: F, t45159: F, t45177: F, t45188: F, t45190: F, t5120: F, t5158: F, t58005: F, t58247: F, t6503: F, t6506: F, t6535: F, t6538: F, t69359: F, t69376: F, t81791: F, t82050: F, t90327: F, t90499: F, t6502: F, t12429: F, t12486: F, t12553: F, t17032: F, t17097: F, t24431: F, t24436: F, t3452: F, t3477: F, t3479: F, t3496: F, t3521: F, t3523: F, t6487: F, t69488: F, t90319: F, t90329: F, t90332: F, t90336: F, t90339: F, t90341: F, t90343: F, t90352: F, t17023: F, t1744: F, t1756: F, t20678: F, t24417: F, t24420: F, t6519: F, t6534: F, t81873: F, t90346: F, t90349: F, t90351: F, t90364: F, t90367: F, t90370: F, t90373: F, t45232: F, t12470: F, t17154: F, t20625: F, t24331: F, t24376: F, t24414: F, t24423: F, t435: F, t58262: F, t58304: F, t69371: F, t81836: F, t90505: F, t90509: F, t90511: F, t90514: F, t90578: F, t90580: F, t90582: F, t90585: F, t300: F, t24488: F, t20890: F, t69511: F, t6555: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t90626 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1769::<F>(t45000, t56236, t68257, t68399, t81230, t81232, t81234, t81236, t89865, t89869, t89873, t89877);
        let (t90629, t90631, t90634, t90636, t90640) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1770::<F>(t422, t90614, t90626, t20400, t6556, t1196, t24408, t5197, t24473, t5192, t1188, t12485, t90357);
        let (t90644, t90670, t90688) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1771::<F>(t1196, t12555, t43752, t90357, t6486, t68255, t68257, t81156, t81158, t89839, t89851, t89865, t89869, t89873, t89877, t90379, t90384, t90387, t90390);
        let t90701 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1772::<F>(t45106, t45107, t89824, t89832, t90402, t90405, t90408, t90411, t90414, t90417, t90420, t90423, t90451, t90453);
        let t90717 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1773::<F>(t81230, t81232, t81234, t81425, t81427, t81429, t89828, t89843, t89847, t89855, t90459, t90464, t90470, t90473);
        let t90732 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1774::<F>(t56236, t58153, t68399, t68583, t68585, t68590, t81236, t81491, t81496, t81539, t90486, t90488, t90490, t90492);
        let t90745 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1775::<F>(t1161, t1169, t1180, t1188, t12472, t12555, t1745, t1757, t20526, t20542, t24363, t24366, t24408, t24411, t45085, t45157, t45159, t45177, t45188, t45190, t5120, t5158, t58005, t58247, t6503, t6506, t6535, t6538, t69359, t69376, t81791, t82050, t90327, t90357, t90499, t90670, t90688, t90701, t90717, t90732);
        let t90775 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1776::<F>(t6502, t1169, t1188, t12429, t12486, t12553, t17032, t17097, t24431, t24436, t3452, t3477, t3479, t3496, t3521, t3523, t6487, t69488, t90319, t90329, t90332, t90336, t90339, t90341, t90343, t90352, t90357, t90670);
        let t90805 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1777::<F>(t12429, t12486, t12553, t17023, t17032, t1744, t1756, t20678, t24363, t24408, t24417, t24420, t3452, t3477, t3496, t3521, t6487, t6502, t6506, t6519, t6534, t6538, t81873, t90346, t90349, t90351, t90364, t90367, t90370, t90373);
        let (t90836, t90848) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1778::<F>(t68255, t81156, t81158, t89824, t89828, t89832, t89839, t89843, t89847, t89851, t89855, t45232, t56236, t68257, t68399, t81230, t81232, t81234, t81236, t89865, t89869, t89873, t89877);
        let t90852 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1779::<F>(t12470, t17097, t17154, t1744, t20625, t24331, t24376, t24414, t24423, t3477, t3479, t435, t58262, t58304, t6502, t6519, t69371, t81836, t90505, t90509, t90511, t90514, t90578, t90580, t90582, t90585, t90629, t90670, t90836, t90848);
        let (t90855, t90857, t90860, t90863) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1780::<F>(t300, t90745, t90775, t90805, t90852, t24488, t5192, t1196, t20890, t69511, t6535, t6555);
    (t90629, t90631, t90634, t90636, t90640, t90644, t90855, t90857, t90860, t90863)
}
