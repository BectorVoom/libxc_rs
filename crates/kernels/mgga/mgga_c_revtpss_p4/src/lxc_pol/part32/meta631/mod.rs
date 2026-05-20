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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta631<F: Float>(t670: F, t7968: F, t102019: F, t109150: F, t109368: F, t110054: F, t110110: F, t1312: F, t1518: F, t18245: F, t2055: F, t21881: F, t26399: F, t28653: F, t28658: F, t34251: F, t4292: F, t5920: F, t7359: F, t7373: F, t75439: F, t85360: F, t5883: F, t7356: F, t108710: F, t109153: F, t109242: F, t13426: F, t18227: F, t2322: F, t27123: F, t28219: F, t28683: F, t30138: F, t30143: F, t30570: F, t4248: F, t5523: F, t7889: F, t7983: F, t102070: F, t109096: F, t110853: F, t111004: F, t118: F, t13648: F, t2014: F, t2089: F, t21814: F, t21891: F, t22287: F, t22496: F, t25082: F, t26405: F, t26411: F, t27833: F, t28167: F, t28196: F, t28711: F, t28932: F, t29494: F, t30209: F, t30315: F, t34495: F, t569: F, t5877: F, t5887: F, t671: F, t7235: F, t7474: F, t7732: F, t7898: F, t8108: F, t8111: F, t86771: F, t9069: F, t109118: F, t1310: F, t2093: F, t21658: F, t22483: F, t22506: F, t28737: F, t28760: F, t28939: F, t29506: F, t30558: F, t30563: F, t30581: F, t30589: F, t30617: F, t4254: F, t508: F, t651: F, t7374: F, t7488: F, t7489: F, t7536: F, t7978: F, t108714: F, t109100: F, t18242: F, t1843: F, t2056: F, t2107: F, t27126: F, t28286: F, t28704: F, t29508: F, t30218: F, t30511: F, t30586: F, t5921: F, t73407: F, t7367: F, t7984: F, t8107: F, t9593: F, t109077: F, t109104: F, t1453: F, t2108: F, t27153: F, t28198: F, t28588: F, t28709: F, t30122: F, t30584: F, t30612: F, t33183: F, t35927: F, t5627: F, t6934: F, t7484: F, t7537: F, t7539: F, t8109: F, t86815: F, t98450: F, t30313: F, t531: F, t102769: F, t108682: F, t109269: F, t1519: F, t22475: F, t28287: F, t28696: F, t28734: F, t28926: F, t28927: F, t30513: F, t30614: F, t4257: F, t4293: F, t4297: F, t5542: F, t7238: F, t7900: F, t8079: F, t95088: F, t105892: F, t109199: F, t1502: F, t18220: F, t18232: F, t1911: F, t2052: F, t22279: F, t28176: F, t28586: F, t28652: F, t28686: F, t28707: F, t28718: F, t28929: F, t28938: F, t30314: F, t4246: F, t5517: F, t5787: F, t5884: F, t6765: F, t7315: F, t7357: F, t7969: F, t8065: F, t8075: F, t86753: F, t110058: F, t110102: F, t116: F, t117: F, t1459: F, t1916: F, t2113: F, t2115: F, t22544: F, t22559: F, t22565: F, t28975: F, t28981: F, t28987: F, t28990: F, t30654: F, t30657: F, t34359: F, t572: F, t573: F, t5795: F, t5802: F, t6941: F, t6945: F, t7547: F, t7554: F, t8118: F, t8124: F, param_d: F, t101705: F, t1461: F, t1918: F, t22556: F, t22568: F, t26733: F, t28956: F, t28974: F, t28978: F, t28986: F, t30637: F, t30651: F, t30660: F, t5805: F, t6948: F, t7553: F, t7557: F, t8127: F, t2118: F, t6936: F, t104062: F, t1456: F, t1458: F, t1464: F, t1914: F, t1921: F, t2111: F, t22533: F, t22571: F, t28945: F, t28993: F, t3: F, t30627: F, t30663: F, t575: F, t5790: F, t5808: F, t6937: F, t7560: F, t8114: F, t8130: F, t1913: F, t2110: F, t6951: F, t30626: F, t8113: F, t571: F, t104071: F, t104073: F, t104077: F, t104079: F, t104081: F, t104083: F, t104085: F, t7542: F) -> (F, F) {
        let (t111018, t111039) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2043::<F>(t670, t7968, t102019, t109150, t109368, t110054, t110110, t1312, t1518, t18245, t2055, t21881, t26399, t28653, t28658, t34251, t4292, t5920, t7359, t7373, t75439, t85360);
        let (t111066, t111068) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2044::<F>(t5883, t7356, t108710, t109153, t109242, t13426, t18227, t2055, t2322, t27123, t28219, t28683, t30138, t30143, t30570, t4248, t5523, t7373, t7889, t7983);
        let t111089 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2045::<F>(t102070, t109096, t110110, t110853, t111004, t111039, t111068, t118, t13648, t2014, t2089, t21814, t21891, t22287, t22496, t2322, t25082, t26399, t26405, t26411, t27833, t28167, t28196, t28658, t28711, t28932, t29494, t30209, t30315, t34495, t569, t5877, t5887, t671, t7235, t7359, t7474, t7732, t7898, t8108, t8111, t86771, t9069);
        let t111130 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2046::<F>(t109118, t111066, t1310, t13426, t18227, t2014, t2055, t2093, t21658, t22483, t22506, t2322, t28737, t28760, t28939, t29506, t30138, t30558, t30563, t30581, t30589, t30617, t4248, t4254, t508, t5920, t651, t7235, t7374, t7474, t7488, t7489, t7536, t7898, t7978);
        let t111174 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2047::<F>(t108710, t108714, t109100, t13426, t18227, t18242, t1843, t2014, t2056, t2107, t25082, t26399, t27123, t27126, t28286, t28658, t28683, t28704, t28711, t29508, t30218, t30511, t30586, t4248, t5921, t651, t670, t7235, t73407, t7359, t7367, t7732, t7984);
        let t111214 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2048::<F>(t8107, t9593, t109077, t109104, t109150, t109153, t1453, t18245, t2056, t2108, t25082, t26405, t27153, t27833, t28167, t28196, t28198, t28588, t28709, t29506, t30122, t30138, t30584, t30612, t33183, t34495, t35927, t5627, t6934, t7235, t7367, t7374, t7484, t7537, t7539, t7898, t8109, t86815, t98450);
        let t111260 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2049::<F>(t30313, t531, t102019, t102769, t108682, t109269, t111018, t1519, t2014, t22475, t2322, t25082, t26405, t27833, t28287, t28653, t28696, t28734, t28926, t28927, t30513, t30558, t30614, t4248, t4257, t4293, t4297, t5542, t7235, t7238, t7536, t7732, t7898, t7900, t8079, t95088);
        let t111301 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2050::<F>(t105892, t109199, t1502, t18220, t18232, t1843, t1911, t2014, t2052, t2089, t21658, t22279, t25082, t26405, t28167, t28176, t28286, t28586, t28652, t28686, t28707, t28718, t28929, t28938, t30314, t4246, t5517, t5787, t5884, t6765, t7315, t7357, t7359, t7474, t7898, t7969, t8065, t8075, t86753, t9069, t98450);
        let (t111304, t111345) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2051::<F>(t110058, t110102, t111089, t111130, t111174, t111214, t111260, t111301, t116, t30570, t109368, t117, t1459, t1916, t2113, t2115, t22544, t22559, t22565, t28975, t28981, t28987, t28990, t30654, t30657, t34359, t4292, t572, t573, t5795, t5802, t670, t6941, t6945, t7547, t7554, t8118, t8124, param_d);
        let t111390 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2052::<F>(t670, t7983, t101705, t1459, t1461, t1518, t1916, t1918, t2113, t21881, t22556, t22568, t26733, t28956, t28974, t28978, t28986, t30637, t30651, t30660, t4292, t572, t5795, t5805, t5883, t5920, t6941, t6948, t7373, t7547, t7553, t7557, t8118, t8127);
        let t111407 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2053::<F>(t2118, t6936, t104062, t111304, t111345, t111390, t1456, t1458, t1464, t1914, t1921, t2111, t22533, t22571, t28945, t28993, t3, t30627, t30663, t575, t5790, t5808, t6937, t7560, t8114, t8130);
        let t111416 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2054::<F>(t1913, t8130, t2110, t6951, t30626, t575, t1921, t8113, t30663, t571, t104071, t104073, t104077, t104079, t104081, t104083, t104085, t7542);
    (t111407, t111416)
}
