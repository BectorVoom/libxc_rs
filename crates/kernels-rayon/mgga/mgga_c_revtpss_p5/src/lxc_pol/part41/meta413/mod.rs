//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta413 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1455;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1456;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1457;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1458;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1459;
use chunk5::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1460;
use chunk6::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1461;
use chunk7::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1462;
use chunk8::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1463;
use chunk9::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1464;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta413(t1904: f64, t5599: f64, t689: f64, t10157: f64, t14091: f64, t14096: f64, t14097: f64, t14102: f64, t14105: f64, t14108: f64, t14111: f64, t14276: f64, t5715: f64, t5728: f64, t9694: f64, t9695: f64, t1444: f64, t6895: f64, t9657: f64, t22307: f64, t225: f64, t212: f64, t6888: f64, t1358: f64, t1357: f64, t6896: f64, t72: f64, t686: f64, t9680: f64, t10160: f64, t10163: f64, t10166: f64, t1424: f64, t14280: f64, t14290: f64, t14294: f64, t14297: f64, t213: f64, t4071: f64, t561: f64, t6919: f64, t22393: f64, t22418: f64, t1343: f64, t1353: f64, t13599: f64, t13600: f64, t1450: f64, t1868: f64, t198: f64, t21901: f64, t21905: f64, t21933: f64, t21937: f64, t21969: f64, t4139: f64, t532: f64, t5532: f64, t5536: f64, t5591: f64, t5627: f64, t9278: f64, t9308: f64, t9316: f64, t9320: f64, t9325: f64, t9329: f64, t9333: f64, t9374: f64, t9389: f64, t9391: f64, t4147: f64, t6781: f64, t4140: f64, t6836: f64, t13615: f64, t13620: f64, t13623: f64, t13634: f64, t13635: f64, t22187: f64, t22189: f64, t22192: f64, t22194: f64, t22196: f64, t22197: f64, t22198: f64, t22199: f64, t22200: f64, t22201: f64, t22202: f64, t9394: f64, t9415: f64, t9593: f64, t5537: f64, t13643: f64, t1448: f64, t22205: f64, t22206: f64, t22207: f64, t22208: f64, t22209: f64, t22211: f64, t5541: f64, t9421: f64, t9427: f64, t9429: f64, t9514: f64, t9517: f64, t9521: f64, t9546: f64, t9569: f64, t9574: f64, t9577: f64, t9588: f64, t6922: f64, t566: f64, t6816: f64, t13664: f64, t13682: f64, t13683: f64, t22214: f64, t22215: f64, t22216: f64, t22217: f64, t22218: f64, t22219: f64, t5542: f64, t5778: f64, t9524: f64, t9542: f64, t9854: f64, t9865: f64, t9868: f64, t1312: f64, t13426: f64, t1518: f64, t18220: f64, t18227: f64, t18245: f64, t21814: f64, t21881: f64, t2322: f64, t4248: f64, t4292: f64, t5523: f64, t5920: f64, t670: f64, t7889: f64, t1315: f64, t1453: f64, t1847: f64, t1911: f64, t21882: f64, t21891: f64, t4254: f64, t4293: f64, t4297: f64, t508: f64, t511: f64, t5528: f64, t569: f64, t5787: f64, t5887: f64, t649: f64, t651: f64, t6765: f64, t6773: f64, t6934: f64, t7732: f64, t21660: f64, t3: f64, t5883: f64, t5801: f64, t116: f64, t117: f64, t1459: f64, t1461: f64, t1916: f64, t1918: f64, t572: f64, t573: f64, t5795: f64, t5802: f64, t5805: f64, t6941: f64, t6945: f64, t6948: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t22430 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1455(t1904, t5599, t689, t10157, t14091, t14096, t14097, t14102, t14105, t14108, t14111, t14276, t5715, t5728, t9694, t9695);
        let (t22433, t22441, t22447, t22450, t22452) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1456(t1444, t6895, t9657, t22307, t225, t212, t6888, t1358, t689, t1357, t6896, t72);
        let t22459 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1457(t22452, t686, t9680, t10160, t10163, t10166, t1424, t14280, t14290, t14294, t14297, t213, t22433, t22441, t22447, t22450, t4071, t561, t6919);
        let t22465 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1458(t22393, t22418, t22430, t22459, t1343, t1353, t13599, t13600, t1450, t1868, t198, t21901, t21905, t21933, t21937, t21969, t4139, t532, t5532, t5536, t5591, t5627, t9278, t9308, t9316, t9320, t9325, t9329, t9333, t9374, t9389, t9391);
        let t22473 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1459(t4147, t6781, t4140, t6836, t1353, t13615, t13620, t13623, t13634, t13635, t22187, t22189, t22192, t22194, t22196, t22197, t22198, t22199, t22200, t22201, t22202, t4139, t5536, t9394, t9415);
        let t22482 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1460(t6781, t9593, t5537, t5591, t13643, t1448, t22205, t22206, t22207, t22208, t22209, t22211, t5536, t5541, t9421, t9427, t9429, t9514, t9517, t9521, t9546, t9569, t9574, t9577, t9588);
        let t22504 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1461(t4147, t6922, t566, t6816, t1448, t1868, t1353, t13664, t13682, t13683, t198, t22214, t22215, t22216, t22217, t22218, t22219, t4139, t4140, t5536, t5541, t5542, t5778, t6836, t9524, t9542, t9854, t9865, t9868);
        let (t22506, t22525) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1462(t22465, t22473, t22482, t22504, t1312, t13426, t1518, t18220, t18227, t18245, t21814, t21881, t2322, t4248, t4292, t5523, t5920, t670, t7889);
        let t22531 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1463(t1315, t1453, t1847, t1911, t21814, t21882, t21891, t22506, t22525, t2322, t4248, t4254, t4293, t4297, t508, t511, t5528, t569, t5787, t5887, t649, t651, t6765, t6773, t6934, t7732);
        let (t22533, t22544, t22556, t22559, t22565, t22568, t22571) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1464(t21660, t22531, t3, t5883, t670, t4292, t5801, t116, t5920, t117, t21881, t1459, t1461, t1916, t1918, t572, t573, t5795, t5802, t5805, t6941, t6945, t6948, param_d);
    (t22506, t22533, t22544, t22556, t22559, t22565, t22568, t22571)
}
