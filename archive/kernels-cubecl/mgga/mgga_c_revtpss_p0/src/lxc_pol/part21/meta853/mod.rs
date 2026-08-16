//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta853 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3212;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3213;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3214;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3215;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3216;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3217;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3218;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3219;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3220;
use chunk9::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3221;
use chunk10::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3222;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta853<F: Float>(t3655: F, t5258: F, t5262: F, t12976: F, t5362: F, t12963: F, t5327: F, t12995: F, t17308: F, t12966: F, t1803: F, t17283: F, t3678: F, t12901: F, t17572: F, t17235: F, t372: F, t13068: F, t5292: F, t1032: F, t1246: F, t17331: F, t1247: F, t17221: F, t3172: F, t17583: F, t3711: F, t1042: F, t1252: F, t1261: F, t12621: F, t12889: F, t1469: F, t17550: F, t17693: F, t225: F, t3674: F, t45382: F, t45389: F, t480: F, t484: F, t5296: F, t53450: F, t56479: F, t57622: F, t127: F, t17695: F, t5268: F, t17708: F, t45779: F, t13089: F, t5391: F, t13085: F, t5381: F, t1284: F, t17306: F, t3624: F, t12916: F, t17704: F, t5340: F, t12898: F, t1804: F, t12948: F, t17529: F, t11262: F, t5278: F, t1122: F, t11231: F, t1214: F, t12832: F, t12912: F, t12931: F, t12933: F, t13312: F, t16775: F, t17212: F, t17505: F, t17552: F, t17649: F, t17729: F, t17736: F, t17744: F, t17750: F, t21035: F, t3584: F, t3626: F, t3647: F, t4186: F, t44521: F, t5051: F, t5405: F, t56765: F, t56818: F, t56873: F, t56932: F, t56985: F, t57047: F, t57103: F, t57150: F, t57193: F, t57254: F, t57308: F, t57370: F, t57433: F, t57496: F, t57555: F, t57610: F, t57667: F, t57728: F, t57779: F, t58772: F, t58842: F, t58886: F, t58948: F, t59007: F, t59056: F, t59108: F, t59151: F, t59215: F, t59267: F, t59334: F, t12640: F, t1811: F, t3601: F, t5412: F, t17807: F, t473: F, t3766: F, t5216: F, t13141: F, t1770: F, t1234: F, t1248: F, t1285: F, t1287: F, t1291: F, t12987: F, t13144: F, t17345: F, t17822: F, t17917: F, t3666: F, t3727: F, t3759: F, t3767: F, t3769: F, t3770: F, t460: F, t487: F, t489: F, t5284: F, t58730: F, t17482: F, t3153: F, t12646: F, t12699: F, t12702: F, t12734: F, t12737: F, t1288: F, t13133: F, t13156: F, t17307: F, t17811: F, t17815: F, t21483: F, t3670: F, t3751: F, t45859: F, t5230: F, t5436: F, t5463: F, t5465: F, t5470: F, t5478: F, t5480: F, t5486: F, t57373: F, t59241: F, t13126: F, t12690: F, t12732: F, t13108: F, t13121: F, t13130: F, t16750: F, t17821: F, t17861: F, t1825: F, t3568: F, t3755: F, t3778: F, t5245: F, t5326: F, t57200: F, t57498: F, t1269: F, t12709: F, t12723: F, t12727: F, t16771: F, t16772: F, t17192: F, t17840: F, t17846: F, t17848: F, t17856: F, t17884: F, t17888: F, t17902: F, t45868: F, t490: F, t5446: F, t57536: F, t12629: F, t12756: F, t13129: F, t16695: F, t17289: F, t17834: F, t21472: F, t3302: F, t3760: F, t3781: F, t45683: F, t45697: F, t45700: F, t45715: F, t45738: F, t45863: F, t471: F, t5332: F, t5466: F, t5481: F, t57737: F, t59096: F) -> (F, F, F, F, F, F, F, F) {
        let (t59337, t59339, t59349, t59351, t59353, t59355, t59358) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3212::<F>(t3655, t5258, t5262, t12976, t5362, t12963, t5327, t12995, t17308, t12966, t1803, t17283, t3678);
        let (t59360, t59362, t59371, t59375, t59379) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3213::<F>(t12901, t17572, t17235, t372, t13068, t5292, t1032, t1246, t17331, t1247, t17221, t3172);
        let t59388 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3214::<F>(t17583, t3172, t3711, t1042, t1252, t1261, t12621, t12889, t1469, t17550, t17693, t1803, t225, t3674, t45382, t45389, t480, t484, t5296, t53450, t56479, t57622, t59337, t59339, t59349, t59351, t59353, t59355, t59358, t59360, t59362, t59371, t59375, t59379);
        let (t59391, t59401, t59404, t59406, t59408, t59411) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3215::<F>(t127, t17693, t17695, t5268, t17708, t45779, t13089, t5391, t13085, t5381, t1284, t17306, t3624);
        let t59448 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3216::<F>(t12916, t17704, t5340, t12898, t1804, t12948, t17529, t11262, t3711, t5278, t1042, t1122, t11231, t1214, t12832, t12912, t12931, t12933, t13312, t16775, t17212, t17505, t17552, t17649, t17729, t17736, t17744, t17750, t21035, t3584, t3626, t3647, t4186, t44521, t5051, t5296, t5405, t59391, t59401, t59404, t59406, t59408, t59411);
        let t59453 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3217::<F>(t56765, t56818, t56873, t56932, t56985, t57047, t57103, t57150, t57193, t57254, t57308, t57370, t57433, t57496, t57555, t57610, t57667, t57728, t57779, t58772, t58842, t58886, t58948, t59007, t59056, t59108, t59151, t59215, t59267, t59334, t59388, t59448);
        let (t59464, t59476, t59510) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3218::<F>(t12640, t1811, t3601, t5412, t17807, t473, t3766, t5216, t13141, t1770, t1214, t1234, t1248, t1285, t1287, t1291, t12966, t12987, t13144, t17331, t17345, t17822, t17917, t3666, t3727, t3759, t3767, t3769, t3770, t460, t487, t489, t5284, t58730, t59453);
        let (t59514, t59544) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3219::<F>(t17482, t3153, t1284, t17331, t12646, t12699, t12702, t12734, t12737, t1288, t12966, t13133, t13156, t17307, t17811, t17815, t21483, t3670, t3751, t45859, t5230, t5436, t5463, t5465, t5470, t5478, t5480, t5486, t57373, t59241);
        let t59579 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3220::<F>(t13126, t1770, t1234, t12621, t12690, t12732, t1285, t1287, t13108, t13121, t13130, t13133, t16750, t17821, t17861, t1811, t1825, t3568, t3670, t3755, t3759, t3778, t5245, t5326, t5486, t57200, t57498);
        let t59611 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3221::<F>(t1269, t13141, t460, t12709, t12723, t12727, t12966, t16771, t16772, t16775, t17192, t17840, t17846, t17848, t17856, t17884, t17888, t17902, t3670, t3759, t45868, t490, t5446, t56479, t57536);
        let t59649 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3222::<F>(t12629, t12732, t12756, t12987, t13129, t16695, t17289, t17834, t21472, t3302, t3727, t3760, t3766, t3781, t45683, t45697, t45700, t45715, t45738, t45863, t460, t471, t5332, t5446, t5466, t5478, t5481, t5486, t57737, t59096, t59514);
    (t59453, t59464, t59476, t59510, t59544, t59579, t59611, t59649)
}
