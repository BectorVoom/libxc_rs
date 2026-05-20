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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta426<F: Float>(t1921: F, t8283: F, t1455: F, t8389: F, t31619: F, t571: F, t2184: F, t6951: F, t2192: F, t6936: F, t117369: F, t117374: F, t117772: F, t117774: F, t1464: F, t1914: F, t2185: F, t22571: F, t31377: F, t31583: F, t5790: F, t5808: F, t8284: F, t8373: F, t108710: F, t109150: F, t109153: F, t109242: F, t1310: F, t1312: F, t1453: F, t18245: F, t2178: F, t2179: F, t2181: F, t22506: F, t2322: F, t27123: F, t28219: F, t30138: F, t30143: F, t31293: F, t31314: F, t31555: F, t31556: F, t4248: F, t4254: F, t5787: F, t651: F, t7732: F, t7889: F, t8254: F, t8274: F, t8278: F, t8280: F, t8362: F, t8367: F, t116912: F, t31538: F, t105880: F, t116946: F, t117450: F, t117457: F, t117460: F, t117462: F, t117470: F, t117473: F, t117482: F, t117484: F, t117497: F, t117510: F, t21850: F, t21876: F, t31035: F, t31039: F, t31058: F, t5823: F, t5891: F, t5895: F, t5915: F, t658: F, t665: F, t8258: F, t8259: F, t8267: F, t8268: F, t31027: F, t31545: F, t31032: F, t31548: F, t31551: F, t31542: F, t1513: F, t2: F, t105872: F, t105875: F, t116919: F, t116927: F, t116930: F, t116942: F, t116968: F, t116969: F, t117499: F, t117500: F, t117505: F, t117544: F, t1504: F, t21839: F, t2349: F, t31054: F, t31276: F, t31283: F, t31287: F, t31541: F, t4287: F, t114: F, t13426: F, t18227: F, t1843: F, t21658: F, t31248: F, t31292: F, t31299: F, t31320: F, t31324: F, t31518: F, t31570: F, t31579: F, t508: F, t5517: F, t75439: F, t8353: F, t27126: F, t31309: F, t31318: F, t6765: F, t8273: F, t8363: F, t8369: F, t85360: F, t108714: F, t1911: F, t29508: F, t31533: F, t31567: F, t5523: F, t569: F, t6934: F, t670: F, t116: F, t117758: F, t1459: F, t1518: F, t1916: F, t1918: F, t2187: F, t22559: F, t22568: F, t31340: F, t31365: F, t31610: F, t31613: F, t31616: F, t35739: F, t4292: F, t572: F, t573: F, t5795: F, t5805: F, t6941: F, t6948: F, t8289: F, t8299: F, t8377: F, t8383: F, t8386: F, param_d: F, t117: F, t1461: F, t21881: F, t2189: F, t22544: F, t22556: F, t22565: F, t31117: F, t31358: F, t31359: F, t31362: F, t31370: F, t31371: F, t31374: F, t31593: F, t31607: F, t5802: F, t5883: F, t5920: F, t6945: F, t8295: F, t8296: F, t8372: F, t31582: F, t575: F, t1913: F, t117781: F, t117783: F, t1456: F, t1458: F, t22533: F, t3: F, t31329: F, t6937: F, t8302: F) -> F {
        let (t117789, t117793, t118217) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1486::<F>(t1921, t8283, t1455, t8389, t31619, t571, t2184, t6951, t2192, t6936, t117369, t117374, t117772, t117774, t1464, t1914, t2185, t22571, t31377, t31583, t5790, t5808, t8284, t8373);
        let t118276 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1487::<F>(t108710, t109150, t109153, t109242, t1310, t1312, t1453, t18245, t2178, t2179, t2181, t22506, t2322, t27123, t28219, t30138, t30143, t31293, t31314, t31555, t31556, t4248, t4254, t5787, t651, t7732, t7889, t8254, t8274, t8278, t8280, t8362, t8367);
        let t118353 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1488::<F>(t116912, t31538, t105880, t116946, t117450, t117457, t117460, t117462, t117470, t117473, t117482, t117484, t117497, t117510, t21850, t21876, t31035, t31039, t31058, t5823, t5891, t5895, t5915, t658, t665, t8258, t8259, t8267, t8268);
        let t118405 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1489::<F>(t31027, t31545, t31032, t31548, t31551, t31542, t1513, t2, t105872, t105875, t116919, t116927, t116930, t116942, t116968, t116969, t117499, t117500, t117505, t117544, t1504, t21839, t2349, t31035, t31039, t31054, t31058, t31276, t31283, t31287, t31541, t4287, t5823, t5891, t5895, t5915, t658, t8258, t8259, t8267, t8268);
        let (t118407, t118413) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1490::<F>(t114, t118353, t118405, t13426, t18227, t1843, t21658, t2178, t2181, t2322, t30138, t31248, t31292, t31293, t31299, t31320, t31324, t31518, t31570, t31579, t4248, t4254, t508, t5517, t651, t75439, t7732, t7889, t8274, t8353, t8362, t8367);
        let t118456 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1491::<F>(t13426, t18227, t18245, t2179, t2181, t27123, t27126, t28219, t31248, t31299, t31309, t31318, t31324, t4248, t651, t6765, t75439, t7732, t7889, t8254, t8273, t8278, t8353, t8363, t8369, t85360);
        let t118500 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1492::<F>(t108710, t108714, t109150, t109153, t118407, t1312, t13426, t18227, t1911, t2179, t2181, t2322, t29508, t30138, t30143, t31292, t31309, t31320, t31533, t31567, t31570, t4248, t5523, t569, t6934, t8254, t8273, t8274, t8278, t8280, t8369);
        let (t118502, t118533) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1493::<F>(t118276, t118413, t118456, t118500, t670, t8362, t116, t31555, t117758, t1459, t1518, t1916, t1918, t2187, t22559, t22568, t31340, t31365, t31610, t31613, t31616, t35739, t4292, t572, t573, t5795, t5805, t6941, t6948, t8289, t8299, t8377, t8383, t8386, param_d);
        let t118576 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1494::<F>(t117, t118407, t1459, t1461, t1916, t2187, t21881, t2189, t22544, t22556, t22565, t31117, t31358, t31359, t31362, t31370, t31371, t31374, t31593, t31607, t4292, t572, t5802, t5883, t5920, t6941, t6945, t8273, t8289, t8295, t8296, t8377);
        let t118587 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1495::<F>(t1921, t8372, t31582, t575, t1913, t8389, t117781, t117783, t117789, t117793, t118502, t118533, t118576, t1456, t1458, t2192, t22533, t3, t31329, t31619, t6937, t8302);
        let tv4rho3tau4 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1496::<F>(t118217, t118587);
    tv4rho3tau4
}
