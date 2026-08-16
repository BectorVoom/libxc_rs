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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta853(t3655: f64, t5258: f64, t5262: f64, t12976: f64, t5362: f64, t12963: f64, t5327: f64, t12995: f64, t17308: f64, t12966: f64, t1803: f64, t17283: f64, t3678: f64, t12901: f64, t17572: f64, t17235: f64, t372: f64, t13068: f64, t5292: f64, t1032: f64, t1246: f64, t17331: f64, t1247: f64, t17221: f64, t3172: f64, t17583: f64, t3711: f64, t1042: f64, t1252: f64, t1261: f64, t12621: f64, t12889: f64, t1469: f64, t17550: f64, t17693: f64, t225: f64, t3674: f64, t45382: f64, t45389: f64, t480: f64, t484: f64, t5296: f64, t53450: f64, t56479: f64, t57622: f64, t127: f64, t17695: f64, t5268: f64, t17708: f64, t45779: f64, t13089: f64, t5391: f64, t13085: f64, t5381: f64, t1284: f64, t17306: f64, t3624: f64, t12916: f64, t17704: f64, t5340: f64, t12898: f64, t1804: f64, t12948: f64, t17529: f64, t11262: f64, t5278: f64, t1122: f64, t11231: f64, t1214: f64, t12832: f64, t12912: f64, t12931: f64, t12933: f64, t13312: f64, t16775: f64, t17212: f64, t17505: f64, t17552: f64, t17649: f64, t17729: f64, t17736: f64, t17744: f64, t17750: f64, t21035: f64, t3584: f64, t3626: f64, t3647: f64, t4186: f64, t44521: f64, t5051: f64, t5405: f64, t56765: f64, t56818: f64, t56873: f64, t56932: f64, t56985: f64, t57047: f64, t57103: f64, t57150: f64, t57193: f64, t57254: f64, t57308: f64, t57370: f64, t57433: f64, t57496: f64, t57555: f64, t57610: f64, t57667: f64, t57728: f64, t57779: f64, t58772: f64, t58842: f64, t58886: f64, t58948: f64, t59007: f64, t59056: f64, t59108: f64, t59151: f64, t59215: f64, t59267: f64, t59334: f64, t12640: f64, t1811: f64, t3601: f64, t5412: f64, t17807: f64, t473: f64, t3766: f64, t5216: f64, t13141: f64, t1770: f64, t1234: f64, t1248: f64, t1285: f64, t1287: f64, t1291: f64, t12987: f64, t13144: f64, t17345: f64, t17822: f64, t17917: f64, t3666: f64, t3727: f64, t3759: f64, t3767: f64, t3769: f64, t3770: f64, t460: f64, t487: f64, t489: f64, t5284: f64, t58730: f64, t17482: f64, t3153: f64, t12646: f64, t12699: f64, t12702: f64, t12734: f64, t12737: f64, t1288: f64, t13133: f64, t13156: f64, t17307: f64, t17811: f64, t17815: f64, t21483: f64, t3670: f64, t3751: f64, t45859: f64, t5230: f64, t5436: f64, t5463: f64, t5465: f64, t5470: f64, t5478: f64, t5480: f64, t5486: f64, t57373: f64, t59241: f64, t13126: f64, t12690: f64, t12732: f64, t13108: f64, t13121: f64, t13130: f64, t16750: f64, t17821: f64, t17861: f64, t1825: f64, t3568: f64, t3755: f64, t3778: f64, t5245: f64, t5326: f64, t57200: f64, t57498: f64, t1269: f64, t12709: f64, t12723: f64, t12727: f64, t16771: f64, t16772: f64, t17192: f64, t17840: f64, t17846: f64, t17848: f64, t17856: f64, t17884: f64, t17888: f64, t17902: f64, t45868: f64, t490: f64, t5446: f64, t57536: f64, t12629: f64, t12756: f64, t13129: f64, t16695: f64, t17289: f64, t17834: f64, t21472: f64, t3302: f64, t3760: f64, t3781: f64, t45683: f64, t45697: f64, t45700: f64, t45715: f64, t45738: f64, t45863: f64, t471: f64, t5332: f64, t5466: f64, t5481: f64, t57737: f64, t59096: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t59337, t59339, t59349, t59351, t59353, t59355, t59358) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3212(t3655, t5258, t5262, t12976, t5362, t12963, t5327, t12995, t17308, t12966, t1803, t17283, t3678);
        let (t59360, t59362, t59371, t59375, t59379) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3213(t12901, t17572, t17235, t372, t13068, t5292, t1032, t1246, t17331, t1247, t17221, t3172);
        let t59388 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3214(t17583, t3172, t3711, t1042, t1252, t1261, t12621, t12889, t1469, t17550, t17693, t1803, t225, t3674, t45382, t45389, t480, t484, t5296, t53450, t56479, t57622, t59337, t59339, t59349, t59351, t59353, t59355, t59358, t59360, t59362, t59371, t59375, t59379);
        let (t59391, t59401, t59404, t59406, t59408, t59411) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3215(t127, t17693, t17695, t5268, t17708, t45779, t13089, t5391, t13085, t5381, t1284, t17306, t3624);
        let t59448 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3216(t12916, t17704, t5340, t12898, t1804, t12948, t17529, t11262, t3711, t5278, t1042, t1122, t11231, t1214, t12832, t12912, t12931, t12933, t13312, t16775, t17212, t17505, t17552, t17649, t17729, t17736, t17744, t17750, t21035, t3584, t3626, t3647, t4186, t44521, t5051, t5296, t5405, t59391, t59401, t59404, t59406, t59408, t59411);
        let t59453 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3217(t56765, t56818, t56873, t56932, t56985, t57047, t57103, t57150, t57193, t57254, t57308, t57370, t57433, t57496, t57555, t57610, t57667, t57728, t57779, t58772, t58842, t58886, t58948, t59007, t59056, t59108, t59151, t59215, t59267, t59334, t59388, t59448);
        let (t59464, t59476, t59510) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3218(t12640, t1811, t3601, t5412, t17807, t473, t3766, t5216, t13141, t1770, t1214, t1234, t1248, t1285, t1287, t1291, t12966, t12987, t13144, t17331, t17345, t17822, t17917, t3666, t3727, t3759, t3767, t3769, t3770, t460, t487, t489, t5284, t58730, t59453);
        let (t59514, t59544) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3219(t17482, t3153, t1284, t17331, t12646, t12699, t12702, t12734, t12737, t1288, t12966, t13133, t13156, t17307, t17811, t17815, t21483, t3670, t3751, t45859, t5230, t5436, t5463, t5465, t5470, t5478, t5480, t5486, t57373, t59241);
        let t59579 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3220(t13126, t1770, t1234, t12621, t12690, t12732, t1285, t1287, t13108, t13121, t13130, t13133, t16750, t17821, t17861, t1811, t1825, t3568, t3670, t3755, t3759, t3778, t5245, t5326, t5486, t57200, t57498);
        let t59611 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3221(t1269, t13141, t460, t12709, t12723, t12727, t12966, t16771, t16772, t16775, t17192, t17840, t17846, t17848, t17856, t17884, t17888, t17902, t3670, t3759, t45868, t490, t5446, t56479, t57536);
        let t59649 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3222(t12629, t12732, t12756, t12987, t13129, t16695, t17289, t17834, t21472, t3302, t3727, t3760, t3766, t3781, t45683, t45697, t45700, t45715, t45738, t45863, t460, t471, t5332, t5446, t5466, t5478, t5481, t5486, t57737, t59096, t59514);
    (t59453, t59464, t59476, t59510, t59544, t59579, t59611, t59649)
}
