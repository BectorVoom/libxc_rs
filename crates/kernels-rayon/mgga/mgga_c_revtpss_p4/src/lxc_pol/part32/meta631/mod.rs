//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta631 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2043;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2044;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2045;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2046;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2047;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2048;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2049;
use chunk7::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2050;
use chunk8::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2051;
use chunk9::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2052;
use chunk10::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2053;
use chunk11::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2054;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta631(t670: f64, t7968: f64, t102019: f64, t109150: f64, t109368: f64, t110054: f64, t110110: f64, t1312: f64, t1518: f64, t18245: f64, t2055: f64, t21881: f64, t26399: f64, t28653: f64, t28658: f64, t34251: f64, t4292: f64, t5920: f64, t7359: f64, t7373: f64, t75439: f64, t85360: f64, t5883: f64, t7356: f64, t108710: f64, t109153: f64, t109242: f64, t13426: f64, t18227: f64, t2322: f64, t27123: f64, t28219: f64, t28683: f64, t30138: f64, t30143: f64, t30570: f64, t4248: f64, t5523: f64, t7889: f64, t7983: f64, t102070: f64, t109096: f64, t110853: f64, t111004: f64, t118: f64, t13648: f64, t2014: f64, t2089: f64, t21814: f64, t21891: f64, t22287: f64, t22496: f64, t25082: f64, t26405: f64, t26411: f64, t27833: f64, t28167: f64, t28196: f64, t28711: f64, t28932: f64, t29494: f64, t30209: f64, t30315: f64, t34495: f64, t569: f64, t5877: f64, t5887: f64, t671: f64, t7235: f64, t7474: f64, t7732: f64, t7898: f64, t8108: f64, t8111: f64, t86771: f64, t9069: f64, t109118: f64, t1310: f64, t2093: f64, t21658: f64, t22483: f64, t22506: f64, t28737: f64, t28760: f64, t28939: f64, t29506: f64, t30558: f64, t30563: f64, t30581: f64, t30589: f64, t30617: f64, t4254: f64, t508: f64, t651: f64, t7374: f64, t7488: f64, t7489: f64, t7536: f64, t7978: f64, t108714: f64, t109100: f64, t18242: f64, t1843: f64, t2056: f64, t2107: f64, t27126: f64, t28286: f64, t28704: f64, t29508: f64, t30218: f64, t30511: f64, t30586: f64, t5921: f64, t73407: f64, t7367: f64, t7984: f64, t8107: f64, t9593: f64, t109077: f64, t109104: f64, t1453: f64, t2108: f64, t27153: f64, t28198: f64, t28588: f64, t28709: f64, t30122: f64, t30584: f64, t30612: f64, t33183: f64, t35927: f64, t5627: f64, t6934: f64, t7484: f64, t7537: f64, t7539: f64, t8109: f64, t86815: f64, t98450: f64, t30313: f64, t531: f64, t102769: f64, t108682: f64, t109269: f64, t1519: f64, t22475: f64, t28287: f64, t28696: f64, t28734: f64, t28926: f64, t28927: f64, t30513: f64, t30614: f64, t4257: f64, t4293: f64, t4297: f64, t5542: f64, t7238: f64, t7900: f64, t8079: f64, t95088: f64, t105892: f64, t109199: f64, t1502: f64, t18220: f64, t18232: f64, t1911: f64, t2052: f64, t22279: f64, t28176: f64, t28586: f64, t28652: f64, t28686: f64, t28707: f64, t28718: f64, t28929: f64, t28938: f64, t30314: f64, t4246: f64, t5517: f64, t5787: f64, t5884: f64, t6765: f64, t7315: f64, t7357: f64, t7969: f64, t8065: f64, t8075: f64, t86753: f64, t110058: f64, t110102: f64, t116: f64, t117: f64, t1459: f64, t1916: f64, t2113: f64, t2115: f64, t22544: f64, t22559: f64, t22565: f64, t28975: f64, t28981: f64, t28987: f64, t28990: f64, t30654: f64, t30657: f64, t34359: f64, t572: f64, t573: f64, t5795: f64, t5802: f64, t6941: f64, t6945: f64, t7547: f64, t7554: f64, t8118: f64, t8124: f64, param_d: f64, t101705: f64, t1461: f64, t1918: f64, t22556: f64, t22568: f64, t26733: f64, t28956: f64, t28974: f64, t28978: f64, t28986: f64, t30637: f64, t30651: f64, t30660: f64, t5805: f64, t6948: f64, t7553: f64, t7557: f64, t8127: f64, t2118: f64, t6936: f64, t104062: f64, t1456: f64, t1458: f64, t1464: f64, t1914: f64, t1921: f64, t2111: f64, t22533: f64, t22571: f64, t28945: f64, t28993: f64, t3: f64, t30627: f64, t30663: f64, t575: f64, t5790: f64, t5808: f64, t6937: f64, t7560: f64, t8114: f64, t8130: f64, t1913: f64, t2110: f64, t6951: f64, t30626: f64, t8113: f64, t571: f64, t104071: f64, t104073: f64, t104077: f64, t104079: f64, t104081: f64, t104083: f64, t104085: f64, t7542: f64) -> (f64, f64) {
        let (t111018, t111039) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2043(t670, t7968, t102019, t109150, t109368, t110054, t110110, t1312, t1518, t18245, t2055, t21881, t26399, t28653, t28658, t34251, t4292, t5920, t7359, t7373, t75439, t85360);
        let (t111066, t111068) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2044(t5883, t7356, t108710, t109153, t109242, t13426, t18227, t2055, t2322, t27123, t28219, t28683, t30138, t30143, t30570, t4248, t5523, t7373, t7889, t7983);
        let t111089 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2045(t102070, t109096, t110110, t110853, t111004, t111039, t111068, t118, t13648, t2014, t2089, t21814, t21891, t22287, t22496, t2322, t25082, t26399, t26405, t26411, t27833, t28167, t28196, t28658, t28711, t28932, t29494, t30209, t30315, t34495, t569, t5877, t5887, t671, t7235, t7359, t7474, t7732, t7898, t8108, t8111, t86771, t9069);
        let t111130 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2046(t109118, t111066, t1310, t13426, t18227, t2014, t2055, t2093, t21658, t22483, t22506, t2322, t28737, t28760, t28939, t29506, t30138, t30558, t30563, t30581, t30589, t30617, t4248, t4254, t508, t5920, t651, t7235, t7374, t7474, t7488, t7489, t7536, t7898, t7978);
        let t111174 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2047(t108710, t108714, t109100, t13426, t18227, t18242, t1843, t2014, t2056, t2107, t25082, t26399, t27123, t27126, t28286, t28658, t28683, t28704, t28711, t29508, t30218, t30511, t30586, t4248, t5921, t651, t670, t7235, t73407, t7359, t7367, t7732, t7984);
        let t111214 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2048(t8107, t9593, t109077, t109104, t109150, t109153, t1453, t18245, t2056, t2108, t25082, t26405, t27153, t27833, t28167, t28196, t28198, t28588, t28709, t29506, t30122, t30138, t30584, t30612, t33183, t34495, t35927, t5627, t6934, t7235, t7367, t7374, t7484, t7537, t7539, t7898, t8109, t86815, t98450);
        let t111260 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2049(t30313, t531, t102019, t102769, t108682, t109269, t111018, t1519, t2014, t22475, t2322, t25082, t26405, t27833, t28287, t28653, t28696, t28734, t28926, t28927, t30513, t30558, t30614, t4248, t4257, t4293, t4297, t5542, t7235, t7238, t7536, t7732, t7898, t7900, t8079, t95088);
        let t111301 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2050(t105892, t109199, t1502, t18220, t18232, t1843, t1911, t2014, t2052, t2089, t21658, t22279, t25082, t26405, t28167, t28176, t28286, t28586, t28652, t28686, t28707, t28718, t28929, t28938, t30314, t4246, t5517, t5787, t5884, t6765, t7315, t7357, t7359, t7474, t7898, t7969, t8065, t8075, t86753, t9069, t98450);
        let (t111304, t111345) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2051(t110058, t110102, t111089, t111130, t111174, t111214, t111260, t111301, t116, t30570, t109368, t117, t1459, t1916, t2113, t2115, t22544, t22559, t22565, t28975, t28981, t28987, t28990, t30654, t30657, t34359, t4292, t572, t573, t5795, t5802, t670, t6941, t6945, t7547, t7554, t8118, t8124, param_d);
        let t111390 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2052(t670, t7983, t101705, t1459, t1461, t1518, t1916, t1918, t2113, t21881, t22556, t22568, t26733, t28956, t28974, t28978, t28986, t30637, t30651, t30660, t4292, t572, t5795, t5805, t5883, t5920, t6941, t6948, t7373, t7547, t7553, t7557, t8118, t8127);
        let t111407 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2053(t2118, t6936, t104062, t111304, t111345, t111390, t1456, t1458, t1464, t1914, t1921, t2111, t22533, t22571, t28945, t28993, t3, t30627, t30663, t575, t5790, t5808, t6937, t7560, t8114, t8130);
        let t111416 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2054(t1913, t8130, t2110, t6951, t30626, t575, t1921, t8113, t30663, t571, t104071, t104073, t104077, t104079, t104081, t104083, t104085, t7542);
    (t111407, t111416)
}
