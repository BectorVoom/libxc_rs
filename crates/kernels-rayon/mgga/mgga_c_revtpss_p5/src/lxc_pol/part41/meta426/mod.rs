//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta426 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1486;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1487;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1488;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1489;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1490;
use chunk5::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1491;
use chunk6::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1492;
use chunk7::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1493;
use chunk8::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1494;
use chunk9::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1495;
use chunk10::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1496;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta426(t1921: f64, t8283: f64, t1455: f64, t8389: f64, t31619: f64, t571: f64, t2184: f64, t6951: f64, t2192: f64, t6936: f64, t117369: f64, t117374: f64, t117772: f64, t117774: f64, t1464: f64, t1914: f64, t2185: f64, t22571: f64, t31377: f64, t31583: f64, t5790: f64, t5808: f64, t8284: f64, t8373: f64, t108710: f64, t109150: f64, t109153: f64, t109242: f64, t1310: f64, t1312: f64, t1453: f64, t18245: f64, t2178: f64, t2179: f64, t2181: f64, t22506: f64, t2322: f64, t27123: f64, t28219: f64, t30138: f64, t30143: f64, t31293: f64, t31314: f64, t31555: f64, t31556: f64, t4248: f64, t4254: f64, t5787: f64, t651: f64, t7732: f64, t7889: f64, t8254: f64, t8274: f64, t8278: f64, t8280: f64, t8362: f64, t8367: f64, t116912: f64, t31538: f64, t105880: f64, t116946: f64, t117450: f64, t117457: f64, t117460: f64, t117462: f64, t117470: f64, t117473: f64, t117482: f64, t117484: f64, t117497: f64, t117510: f64, t21850: f64, t21876: f64, t31035: f64, t31039: f64, t31058: f64, t5823: f64, t5891: f64, t5895: f64, t5915: f64, t658: f64, t665: f64, t8258: f64, t8259: f64, t8267: f64, t8268: f64, t31027: f64, t31545: f64, t31032: f64, t31548: f64, t31551: f64, t31542: f64, t1513: f64, t2: f64, t105872: f64, t105875: f64, t116919: f64, t116927: f64, t116930: f64, t116942: f64, t116968: f64, t116969: f64, t117499: f64, t117500: f64, t117505: f64, t117544: f64, t1504: f64, t21839: f64, t2349: f64, t31054: f64, t31276: f64, t31283: f64, t31287: f64, t31541: f64, t4287: f64, t114: f64, t13426: f64, t18227: f64, t1843: f64, t21658: f64, t31248: f64, t31292: f64, t31299: f64, t31320: f64, t31324: f64, t31518: f64, t31570: f64, t31579: f64, t508: f64, t5517: f64, t75439: f64, t8353: f64, t27126: f64, t31309: f64, t31318: f64, t6765: f64, t8273: f64, t8363: f64, t8369: f64, t85360: f64, t108714: f64, t1911: f64, t29508: f64, t31533: f64, t31567: f64, t5523: f64, t569: f64, t6934: f64, t670: f64, t116: f64, t117758: f64, t1459: f64, t1518: f64, t1916: f64, t1918: f64, t2187: f64, t22559: f64, t22568: f64, t31340: f64, t31365: f64, t31610: f64, t31613: f64, t31616: f64, t35739: f64, t4292: f64, t572: f64, t573: f64, t5795: f64, t5805: f64, t6941: f64, t6948: f64, t8289: f64, t8299: f64, t8377: f64, t8383: f64, t8386: f64, param_d: f64, t117: f64, t1461: f64, t21881: f64, t2189: f64, t22544: f64, t22556: f64, t22565: f64, t31117: f64, t31358: f64, t31359: f64, t31362: f64, t31370: f64, t31371: f64, t31374: f64, t31593: f64, t31607: f64, t5802: f64, t5883: f64, t5920: f64, t6945: f64, t8295: f64, t8296: f64, t8372: f64, t31582: f64, t575: f64, t1913: f64, t117781: f64, t117783: f64, t1456: f64, t1458: f64, t22533: f64, t3: f64, t31329: f64, t6937: f64, t8302: f64) -> f64 {
        let (t117789, t117793, t118217) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1486(t1921, t8283, t1455, t8389, t31619, t571, t2184, t6951, t2192, t6936, t117369, t117374, t117772, t117774, t1464, t1914, t2185, t22571, t31377, t31583, t5790, t5808, t8284, t8373);
        let t118276 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1487(t108710, t109150, t109153, t109242, t1310, t1312, t1453, t18245, t2178, t2179, t2181, t22506, t2322, t27123, t28219, t30138, t30143, t31293, t31314, t31555, t31556, t4248, t4254, t5787, t651, t7732, t7889, t8254, t8274, t8278, t8280, t8362, t8367);
        let t118353 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1488(t116912, t31538, t105880, t116946, t117450, t117457, t117460, t117462, t117470, t117473, t117482, t117484, t117497, t117510, t21850, t21876, t31035, t31039, t31058, t5823, t5891, t5895, t5915, t658, t665, t8258, t8259, t8267, t8268);
        let t118405 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1489(t31027, t31545, t31032, t31548, t31551, t31542, t1513, t2, t105872, t105875, t116919, t116927, t116930, t116942, t116968, t116969, t117499, t117500, t117505, t117544, t1504, t21839, t2349, t31035, t31039, t31054, t31058, t31276, t31283, t31287, t31541, t4287, t5823, t5891, t5895, t5915, t658, t8258, t8259, t8267, t8268);
        let (t118407, t118413) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1490(t114, t118353, t118405, t13426, t18227, t1843, t21658, t2178, t2181, t2322, t30138, t31248, t31292, t31293, t31299, t31320, t31324, t31518, t31570, t31579, t4248, t4254, t508, t5517, t651, t75439, t7732, t7889, t8274, t8353, t8362, t8367);
        let t118456 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1491(t13426, t18227, t18245, t2179, t2181, t27123, t27126, t28219, t31248, t31299, t31309, t31318, t31324, t4248, t651, t6765, t75439, t7732, t7889, t8254, t8273, t8278, t8353, t8363, t8369, t85360);
        let t118500 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1492(t108710, t108714, t109150, t109153, t118407, t1312, t13426, t18227, t1911, t2179, t2181, t2322, t29508, t30138, t30143, t31292, t31309, t31320, t31533, t31567, t31570, t4248, t5523, t569, t6934, t8254, t8273, t8274, t8278, t8280, t8369);
        let (t118502, t118533) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1493(t118276, t118413, t118456, t118500, t670, t8362, t116, t31555, t117758, t1459, t1518, t1916, t1918, t2187, t22559, t22568, t31340, t31365, t31610, t31613, t31616, t35739, t4292, t572, t573, t5795, t5805, t6941, t6948, t8289, t8299, t8377, t8383, t8386, param_d);
        let t118576 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1494(t117, t118407, t1459, t1461, t1916, t2187, t21881, t2189, t22544, t22556, t22565, t31117, t31358, t31359, t31362, t31370, t31371, t31374, t31593, t31607, t4292, t572, t5802, t5883, t5920, t6941, t6945, t8273, t8289, t8295, t8296, t8377);
        let t118587 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1495(t1921, t8372, t31582, t575, t1913, t8389, t117781, t117783, t117789, t117793, t118502, t118533, t118576, t1456, t1458, t2192, t22533, t3, t31329, t31619, t6937, t8302);
        let tv4rho3tau4 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1496(t118217, t118587);
    tv4rho3tau4
}
