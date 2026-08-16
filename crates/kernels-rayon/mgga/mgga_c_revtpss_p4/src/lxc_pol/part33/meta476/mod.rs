//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta476 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1735;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1736;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1737;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1738;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1739;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1740;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1741;
use chunk7::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1742;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta476(t1903: f64, t5774: f64, t4076: f64, t6918: f64, t72: f64, t686: f64, t3915: f64, t6889: f64, t786: f64, t1364: f64, t14100: f64, t5722: f64, t1357: f64, t6919: f64, t689: f64, t1444: f64, t14081: f64, t14084: f64, t14087: f64, t1424: f64, t14299: f64, t1904: f64, t9677: f64, t9687: f64, t9691: f64, t5599: f64, t10157: f64, t14091: f64, t14096: f64, t14097: f64, t14102: f64, t14105: f64, t14108: f64, t14111: f64, t14276: f64, t5715: f64, t5728: f64, t9694: f64, t9695: f64, t6895: f64, t9657: f64, t22307: f64, t225: f64, t212: f64, t6888: f64, t1358: f64, t6896: f64, t9680: f64, t10160: f64, t10163: f64, t10166: f64, t14280: f64, t14290: f64, t14294: f64, t14297: f64, t213: f64, t4071: f64, t561: f64, t22393: f64, t1343: f64, t1353: f64, t13599: f64, t13600: f64, t1450: f64, t1868: f64, t198: f64, t21901: f64, t21905: f64, t21933: f64, t21937: f64, t21969: f64, t4139: f64, t532: f64, t5532: f64, t5536: f64, t5591: f64, t5627: f64, t9278: f64, t9308: f64, t9316: f64, t9320: f64, t9325: f64, t9329: f64, t9333: f64, t9374: f64, t9389: f64, t9391: f64, t4147: f64, t6781: f64, t4140: f64, t6836: f64, t13615: f64, t13620: f64, t13623: f64, t13634: f64, t13635: f64, t22187: f64, t22189: f64, t22192: f64, t22194: f64, t22196: f64, t22197: f64, t22198: f64, t22199: f64, t22200: f64, t22201: f64, t22202: f64, t9394: f64, t9415: f64, t9593: f64, t5537: f64, t13643: f64, t1448: f64, t22205: f64, t22206: f64, t22207: f64, t22208: f64, t22209: f64, t22211: f64, t5541: f64, t9421: f64, t9427: f64, t9429: f64, t9514: f64, t9517: f64, t9521: f64, t9546: f64, t9569: f64, t9574: f64, t9577: f64, t9588: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22395, t22399, t22400, t22405, t22407) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1735(t1903, t5774, t4076, t6918, t72, t686, t3915, t6889, t786, t1364, t14100, t5722);
        let (t22415, t22418) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1736(t1357, t6919, t689, t1444, t6918, t4076, t14081, t14084, t14087, t1424, t14299, t1904, t22395, t22400, t22405, t22407, t9677, t9687, t9691);
        let t22430 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1737(t1904, t5599, t689, t10157, t14091, t14096, t14097, t14102, t14105, t14108, t14111, t14276, t5715, t5728, t9694, t9695);
        let (t22433, t22441, t22447, t22450, t22452) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1738(t1444, t6895, t9657, t22307, t225, t212, t6888, t1358, t689, t1357, t6896, t72);
        let (t22453, t22459) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1739(t22452, t686, t9680, t10160, t10163, t10166, t1424, t14280, t14290, t14294, t14297, t213, t22433, t22441, t22447, t22450, t4071, t561, t6919);
        let (t22461, t22465) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1740(t22393, t22418, t22430, t22459, t1343, t1353, t13599, t13600, t1450, t1868, t198, t21901, t21905, t21933, t21937, t21969, t4139, t532, t5532, t5536, t5591, t5627, t9278, t9308, t9316, t9320, t9325, t9329, t9333, t9374, t9389, t9391);
        let t22473 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1741(t4147, t6781, t4140, t6836, t1353, t13615, t13620, t13623, t13634, t13635, t22187, t22189, t22192, t22194, t22196, t22197, t22198, t22199, t22200, t22201, t22202, t4139, t5536, t9394, t9415);
        let (t22475, t22482) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1742(t6781, t9593, t5537, t5591, t13643, t1448, t22205, t22206, t22207, t22208, t22209, t22211, t5536, t5541, t9421, t9427, t9429, t9514, t9517, t9521, t9546, t9569, t9574, t9577, t9588);
    (t22395, t22399, t22415, t22433, t22453, t22461, t22465, t22473, t22475, t22482)
}
