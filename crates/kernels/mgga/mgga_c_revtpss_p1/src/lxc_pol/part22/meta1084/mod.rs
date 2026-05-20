//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1084 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3925;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3926;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3927;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3928;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3929;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3930;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3931;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3932;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3933;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3934;
use chunk10::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3935;
use chunk11::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3936;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1084<F: Float>(t5778: F, t198: F, t22466: F, t3829: F, t40076: F, t40079: F, t4147: F, t47152: F, t532: F, t5536: F, t74145: F, t74146: F, t74147: F, t74148: F, t74149: F, t74150: F, t74151: F, t116: F, t21813: F, t118: F, t1310: F, t1315: F, t13544: F, t1453: F, t18235: F, t18245: F, t21881: F, t22506: F, t22525: F, t2322: F, t2328: F, t2331: F, t2371: F, t27123: F, t3813: F, t4151: F, t4254: F, t4292: F, t4293: F, t511: F, t5517: F, t5528: F, t5787: F, t5884: F, t651: F, t671: F, t6765: F, t6773: F, t68231: F, t73306: F, t73326: F, t73343: F, t73359: F, t73376: F, t73383: F, t73400: F, t73417: F, t73495: F, t73528: F, t75357: F, t75372: F, t75386: F, t75401: F, t75408: F, t75412: F, t7732: F, t1501: F, t21830: F, t625: F, t13509: F, t21820: F, t21876: F, t2339: F, t2340: F, t2366: F, t4263: F, t46143: F, t46157: F, t49698: F, t49700: F, t49702: F, t49704: F, t49724: F, t49817: F, t49819: F, t5891: F, t665: F, t69: F, t4287: F, t2289: F, t5916: F, t21877: F, t105: F, t13475: F, t13496: F, t13503: F, t14: F, t1507: F, t21836: F, t21839: F, t21840: F, t21851: F, t21864: F, t21868: F, t21872: F, t22: F, t2344: F, t2349: F, t2350: F, t2357: F, t2359: F, t2362: F, t2363: F, t27: F, t46196: F, t49745: F, t49774: F, t5895: F, t5896: F, t5899: F, t5902: F, t656: F, t661: F, t97: F, t10227: F, t10241: F, t13493: F, t13497: F, t13500: F, t13506: F, t21835: F, t21845: F, t21846: F, t21850: F, t21860: F, t2255: F, t2256: F, t2358: F, t31283: F, t31443: F, t4269: F, t4279: F, t46212: F, t49777: F, t49787: F, t49804: F, t580: F, t5823: F, t5907: F, t5911: F, t658: F, t5892: F, t21821: F, t21824: F, t10208: F, t21829: F, t28036: F, t31035: F, t46144: F, t46146: F, t46148: F, t5915: F, t655: F, t114: F, t4245: F, t670: F, t10416: F, t1312: F, t13426: F, t13435: F, t13440: F, t13514: F, t1518: F, t18227: F, t4248: F, t49686: F, t5523: F, t5920: F, t60650: F, t60656: F, t61010: F, t7889: F, t93: F, t13521: F, t13532: F, t13540: F, t14310: F, t18163: F, t18232: F, t18242: F, t1847: F, t21658: F, t21882: F, t21891: F, t2372: F, t4297: F, t569: F, t5887: F, t5921: F, t13517: F, t13537: F, t1519: F, t1843: F, t1911: F, t3821: F, t4257: F, t508: F, t6934: F, t94: F, t61014: F, t1455: F, t6951: F, t1464: F, t6936: F, t1459: F, t1461: F, t18204: F, t18211: F, t18214: F, t1916: F, t22544: F, t22564: F, t22565: F, t4158: F, t572: F, t5795: F, t5801: F, t5802: F, t5805: F, t6945: F, t6948: F, t117: F, t18190: F, t18208: F, t1918: F, t22556: F, t22559: F, t22568: F, t2327: F, t4162: F, t4165: F, t573: F, t5883: F, t60595: F, t6941: F, param_d: F, t22571: F, t571: F, t1458: F, t18178: F, t18217: F, t1914: F, t1921: F, t4168: F, t5790: F, t5808: F, t60609: F, t60611: F, t60616: F, t60618: F, t6937: F) -> (F, F, F) {
        let t75421 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3925::<F>(t5778, t198, t22466, t3829, t40076, t40079, t4147, t47152, t532, t5536, t74145, t74146, t74147, t74148, t74149, t74150, t74151);
        let (t75439, t75451) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3926::<F>(t116, t21813, t118, t1310, t1315, t13544, t1453, t18235, t18245, t21881, t22506, t22525, t2322, t2328, t2331, t2371, t27123, t3813, t4151, t4254, t4292, t4293, t511, t5517, t5528, t5787, t5884, t651, t671, t6765, t6773, t68231, t73306, t73326, t73343, t73359, t73376, t73383, t73400, t73417, t73495, t73528, t75357, t75372, t75386, t75401, t75408, t75412, t75421, t7732);
        let (t75485, t75494, t75532) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3927::<F>(t1501, t2371, t4292, t21830, t625, t13509, t21820, t21876, t2339, t2340, t2366, t4263, t46143, t46157, t49698, t49700, t49702, t49704, t49724, t49817, t49819, t5891, t665, t69);
        let (t75536, t75540, t75542, t75585) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3928::<F>(t4287, t2289, t5916, t21877, t625, t105, t13475, t13496, t13503, t14, t1507, t21836, t21839, t21840, t21851, t21864, t21868, t21872, t22, t2344, t2349, t2350, t2357, t2359, t2362, t2363, t27, t46196, t49745, t49774, t5895, t5896, t5899, t5902, t656, t661, t97);
        let t75634 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3929::<F>(t1507, t2357, t10227, t10241, t105, t13493, t13497, t13500, t13506, t21835, t21845, t21846, t21850, t21860, t2255, t2256, t2349, t2350, t2358, t2362, t31283, t31443, t4269, t4279, t46212, t49777, t49787, t49804, t580, t5823, t5907, t5911, t656, t658, t97);
        let t75655 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3930::<F>(t2289, t5892, t21821, t625, t21824, t10208, t21829, t2339, t2340, t2366, t28036, t31035, t4287, t46144, t46146, t46148, t5915, t655, t69, t75536, t75540, t75542, t75585, t75634);
        let (t75657, t75667, t75672) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3931::<F>(t114, t75532, t75655, t4245, t670, t10416, t1312, t13426, t13435, t13440, t13514, t1518, t18227, t18245, t21881, t2322, t2371, t27123, t4248, t4292, t49686, t5523, t5920, t60650, t60656, t61010, t75439, t75485, t75494, t7889, t93);
        let t75676 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3932::<F>(t10416, t13426, t13435, t13521, t13532, t13540, t13544, t14310, t18163, t18232, t18242, t18245, t1847, t21658, t21882, t21891, t2322, t2372, t4248, t4254, t4297, t569, t5887, t5921, t651, t670, t75672, t7732);
        let t75714 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3933::<F>(t13426, t13514, t13517, t13537, t1519, t18163, t18227, t1843, t1911, t21882, t21891, t3813, t3821, t4248, t4254, t4257, t4293, t49686, t508, t5887, t5920, t651, t6934, t75485, t75494, t75657, t75667, t94);
        let (t75716, t75720, t75727, t75760) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3934::<F>(t61014, t75451, t75676, t75714, t1455, t6951, t1464, t6936, t116, t13514, t1459, t1461, t18204, t18211, t18214, t1916, t21881, t22544, t22564, t22565, t2371, t4158, t572, t5795, t5801, t5802, t5805, t670, t6945, t6948);
        let t75792 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3935::<F>(t116, t117, t1459, t18190, t18208, t1916, t1918, t22556, t22559, t22568, t2327, t2371, t4162, t4165, t4292, t572, t573, t5883, t5920, t60595, t6941, t75494, t75657, t75716, param_d);
        let t75801 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3936::<F>(t22571, t571, t1458, t18178, t18217, t1914, t1921, t4168, t5790, t5808, t60609, t60611, t60616, t60618, t6937, t75727, t75760, t75792);
    (t75716, t75720, t75801)
}
