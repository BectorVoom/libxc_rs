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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta577(t45000: f64, t56236: f64, t68257: f64, t68399: f64, t81230: f64, t81232: f64, t81234: f64, t81236: f64, t89865: f64, t89869: f64, t89873: f64, t89877: f64, t422: f64, t90614: f64, t20400: f64, t6556: f64, t1196: f64, t24408: f64, t5197: f64, t24473: f64, t5192: f64, t1188: f64, t12485: f64, t90357: f64, t12555: f64, t43752: f64, t6486: f64, t68255: f64, t81156: f64, t81158: f64, t89839: f64, t89851: f64, t90379: f64, t90384: f64, t90387: f64, t90390: f64, t45106: f64, t45107: f64, t89824: f64, t89832: f64, t90402: f64, t90405: f64, t90408: f64, t90411: f64, t90414: f64, t90417: f64, t90420: f64, t90423: f64, t90451: f64, t90453: f64, t81425: f64, t81427: f64, t81429: f64, t89828: f64, t89843: f64, t89847: f64, t89855: f64, t90459: f64, t90464: f64, t90470: f64, t90473: f64, t58153: f64, t68583: f64, t68585: f64, t68590: f64, t81491: f64, t81496: f64, t81539: f64, t90486: f64, t90488: f64, t90490: f64, t90492: f64, t1161: f64, t1169: f64, t1180: f64, t12472: f64, t1745: f64, t1757: f64, t20526: f64, t20542: f64, t24363: f64, t24366: f64, t24411: f64, t45085: f64, t45157: f64, t45159: f64, t45177: f64, t45188: f64, t45190: f64, t5120: f64, t5158: f64, t58005: f64, t58247: f64, t6503: f64, t6506: f64, t6535: f64, t6538: f64, t69359: f64, t69376: f64, t81791: f64, t82050: f64, t90327: f64, t90499: f64, t6502: f64, t12429: f64, t12486: f64, t12553: f64, t17032: f64, t17097: f64, t24431: f64, t24436: f64, t3452: f64, t3477: f64, t3479: f64, t3496: f64, t3521: f64, t3523: f64, t6487: f64, t69488: f64, t90319: f64, t90329: f64, t90332: f64, t90336: f64, t90339: f64, t90341: f64, t90343: f64, t90352: f64, t17023: f64, t1744: f64, t1756: f64, t20678: f64, t24417: f64, t24420: f64, t6519: f64, t6534: f64, t81873: f64, t90346: f64, t90349: f64, t90351: f64, t90364: f64, t90367: f64, t90370: f64, t90373: f64, t45232: f64, t12470: f64, t17154: f64, t20625: f64, t24331: f64, t24376: f64, t24414: f64, t24423: f64, t435: f64, t58262: f64, t58304: f64, t69371: f64, t81836: f64, t90505: f64, t90509: f64, t90511: f64, t90514: f64, t90578: f64, t90580: f64, t90582: f64, t90585: f64, t300: f64, t24488: f64, t20890: f64, t69511: f64, t6555: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t90626 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1769(t45000, t56236, t68257, t68399, t81230, t81232, t81234, t81236, t89865, t89869, t89873, t89877);
        let (t90629, t90631, t90634, t90636, t90640) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1770(t422, t90614, t90626, t20400, t6556, t1196, t24408, t5197, t24473, t5192, t1188, t12485, t90357);
        let (t90644, t90670, t90688) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1771(t1196, t12555, t43752, t90357, t6486, t68255, t68257, t81156, t81158, t89839, t89851, t89865, t89869, t89873, t89877, t90379, t90384, t90387, t90390);
        let t90701 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1772(t45106, t45107, t89824, t89832, t90402, t90405, t90408, t90411, t90414, t90417, t90420, t90423, t90451, t90453);
        let t90717 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1773(t81230, t81232, t81234, t81425, t81427, t81429, t89828, t89843, t89847, t89855, t90459, t90464, t90470, t90473);
        let t90732 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1774(t56236, t58153, t68399, t68583, t68585, t68590, t81236, t81491, t81496, t81539, t90486, t90488, t90490, t90492);
        let t90745 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1775(t1161, t1169, t1180, t1188, t12472, t12555, t1745, t1757, t20526, t20542, t24363, t24366, t24408, t24411, t45085, t45157, t45159, t45177, t45188, t45190, t5120, t5158, t58005, t58247, t6503, t6506, t6535, t6538, t69359, t69376, t81791, t82050, t90327, t90357, t90499, t90670, t90688, t90701, t90717, t90732);
        let t90775 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1776(t6502, t1169, t1188, t12429, t12486, t12553, t17032, t17097, t24431, t24436, t3452, t3477, t3479, t3496, t3521, t3523, t6487, t69488, t90319, t90329, t90332, t90336, t90339, t90341, t90343, t90352, t90357, t90670);
        let t90805 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1777(t12429, t12486, t12553, t17023, t17032, t1744, t1756, t20678, t24363, t24408, t24417, t24420, t3452, t3477, t3496, t3521, t6487, t6502, t6506, t6519, t6534, t6538, t81873, t90346, t90349, t90351, t90364, t90367, t90370, t90373);
        let (t90836, t90848) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1778(t68255, t81156, t81158, t89824, t89828, t89832, t89839, t89843, t89847, t89851, t89855, t45232, t56236, t68257, t68399, t81230, t81232, t81234, t81236, t89865, t89869, t89873, t89877);
        let t90852 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1779(t12470, t17097, t17154, t1744, t20625, t24331, t24376, t24414, t24423, t3477, t3479, t435, t58262, t58304, t6502, t6519, t69371, t81836, t90505, t90509, t90511, t90514, t90578, t90580, t90582, t90585, t90629, t90670, t90836, t90848);
        let (t90855, t90857, t90860, t90863) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1780(t300, t90745, t90775, t90805, t90852, t24488, t5192, t1196, t20890, t69511, t6535, t6555);
    (t90629, t90631, t90634, t90636, t90640, t90644, t90855, t90857, t90860, t90863)
}
