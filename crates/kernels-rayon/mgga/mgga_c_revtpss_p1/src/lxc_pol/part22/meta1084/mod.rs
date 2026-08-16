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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1084(t5778: f64, t198: f64, t22466: f64, t3829: f64, t40076: f64, t40079: f64, t4147: f64, t47152: f64, t532: f64, t5536: f64, t74145: f64, t74146: f64, t74147: f64, t74148: f64, t74149: f64, t74150: f64, t74151: f64, t116: f64, t21813: f64, t118: f64, t1310: f64, t1315: f64, t13544: f64, t1453: f64, t18235: f64, t18245: f64, t21881: f64, t22506: f64, t22525: f64, t2322: f64, t2328: f64, t2331: f64, t2371: f64, t27123: f64, t3813: f64, t4151: f64, t4254: f64, t4292: f64, t4293: f64, t511: f64, t5517: f64, t5528: f64, t5787: f64, t5884: f64, t651: f64, t671: f64, t6765: f64, t6773: f64, t68231: f64, t73306: f64, t73326: f64, t73343: f64, t73359: f64, t73376: f64, t73383: f64, t73400: f64, t73417: f64, t73495: f64, t73528: f64, t75357: f64, t75372: f64, t75386: f64, t75401: f64, t75408: f64, t75412: f64, t7732: f64, t1501: f64, t21830: f64, t625: f64, t13509: f64, t21820: f64, t21876: f64, t2339: f64, t2340: f64, t2366: f64, t4263: f64, t46143: f64, t46157: f64, t49698: f64, t49700: f64, t49702: f64, t49704: f64, t49724: f64, t49817: f64, t49819: f64, t5891: f64, t665: f64, t69: f64, t4287: f64, t2289: f64, t5916: f64, t21877: f64, t105: f64, t13475: f64, t13496: f64, t13503: f64, t14: f64, t1507: f64, t21836: f64, t21839: f64, t21840: f64, t21851: f64, t21864: f64, t21868: f64, t21872: f64, t22: f64, t2344: f64, t2349: f64, t2350: f64, t2357: f64, t2359: f64, t2362: f64, t2363: f64, t27: f64, t46196: f64, t49745: f64, t49774: f64, t5895: f64, t5896: f64, t5899: f64, t5902: f64, t656: f64, t661: f64, t97: f64, t10227: f64, t10241: f64, t13493: f64, t13497: f64, t13500: f64, t13506: f64, t21835: f64, t21845: f64, t21846: f64, t21850: f64, t21860: f64, t2255: f64, t2256: f64, t2358: f64, t31283: f64, t31443: f64, t4269: f64, t4279: f64, t46212: f64, t49777: f64, t49787: f64, t49804: f64, t580: f64, t5823: f64, t5907: f64, t5911: f64, t658: f64, t5892: f64, t21821: f64, t21824: f64, t10208: f64, t21829: f64, t28036: f64, t31035: f64, t46144: f64, t46146: f64, t46148: f64, t5915: f64, t655: f64, t114: f64, t4245: f64, t670: f64, t10416: f64, t1312: f64, t13426: f64, t13435: f64, t13440: f64, t13514: f64, t1518: f64, t18227: f64, t4248: f64, t49686: f64, t5523: f64, t5920: f64, t60650: f64, t60656: f64, t61010: f64, t7889: f64, t93: f64, t13521: f64, t13532: f64, t13540: f64, t14310: f64, t18163: f64, t18232: f64, t18242: f64, t1847: f64, t21658: f64, t21882: f64, t21891: f64, t2372: f64, t4297: f64, t569: f64, t5887: f64, t5921: f64, t13517: f64, t13537: f64, t1519: f64, t1843: f64, t1911: f64, t3821: f64, t4257: f64, t508: f64, t6934: f64, t94: f64, t61014: f64, t1455: f64, t6951: f64, t1464: f64, t6936: f64, t1459: f64, t1461: f64, t18204: f64, t18211: f64, t18214: f64, t1916: f64, t22544: f64, t22564: f64, t22565: f64, t4158: f64, t572: f64, t5795: f64, t5801: f64, t5802: f64, t5805: f64, t6945: f64, t6948: f64, t117: f64, t18190: f64, t18208: f64, t1918: f64, t22556: f64, t22559: f64, t22568: f64, t2327: f64, t4162: f64, t4165: f64, t573: f64, t5883: f64, t60595: f64, t6941: f64, param_d: f64, t22571: f64, t571: f64, t1458: f64, t18178: f64, t18217: f64, t1914: f64, t1921: f64, t4168: f64, t5790: f64, t5808: f64, t60609: f64, t60611: f64, t60616: f64, t60618: f64, t6937: f64) -> (f64, f64, f64) {
        let t75421 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3925(t5778, t198, t22466, t3829, t40076, t40079, t4147, t47152, t532, t5536, t74145, t74146, t74147, t74148, t74149, t74150, t74151);
        let (t75439, t75451) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3926(t116, t21813, t118, t1310, t1315, t13544, t1453, t18235, t18245, t21881, t22506, t22525, t2322, t2328, t2331, t2371, t27123, t3813, t4151, t4254, t4292, t4293, t511, t5517, t5528, t5787, t5884, t651, t671, t6765, t6773, t68231, t73306, t73326, t73343, t73359, t73376, t73383, t73400, t73417, t73495, t73528, t75357, t75372, t75386, t75401, t75408, t75412, t75421, t7732);
        let (t75485, t75494, t75532) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3927(t1501, t2371, t4292, t21830, t625, t13509, t21820, t21876, t2339, t2340, t2366, t4263, t46143, t46157, t49698, t49700, t49702, t49704, t49724, t49817, t49819, t5891, t665, t69);
        let (t75536, t75540, t75542, t75585) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3928(t4287, t2289, t5916, t21877, t625, t105, t13475, t13496, t13503, t14, t1507, t21836, t21839, t21840, t21851, t21864, t21868, t21872, t22, t2344, t2349, t2350, t2357, t2359, t2362, t2363, t27, t46196, t49745, t49774, t5895, t5896, t5899, t5902, t656, t661, t97);
        let t75634 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3929(t1507, t2357, t10227, t10241, t105, t13493, t13497, t13500, t13506, t21835, t21845, t21846, t21850, t21860, t2255, t2256, t2349, t2350, t2358, t2362, t31283, t31443, t4269, t4279, t46212, t49777, t49787, t49804, t580, t5823, t5907, t5911, t656, t658, t97);
        let t75655 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3930(t2289, t5892, t21821, t625, t21824, t10208, t21829, t2339, t2340, t2366, t28036, t31035, t4287, t46144, t46146, t46148, t5915, t655, t69, t75536, t75540, t75542, t75585, t75634);
        let (t75657, t75667, t75672) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3931(t114, t75532, t75655, t4245, t670, t10416, t1312, t13426, t13435, t13440, t13514, t1518, t18227, t18245, t21881, t2322, t2371, t27123, t4248, t4292, t49686, t5523, t5920, t60650, t60656, t61010, t75439, t75485, t75494, t7889, t93);
        let t75676 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3932(t10416, t13426, t13435, t13521, t13532, t13540, t13544, t14310, t18163, t18232, t18242, t18245, t1847, t21658, t21882, t21891, t2322, t2372, t4248, t4254, t4297, t569, t5887, t5921, t651, t670, t75672, t7732);
        let t75714 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3933(t13426, t13514, t13517, t13537, t1519, t18163, t18227, t1843, t1911, t21882, t21891, t3813, t3821, t4248, t4254, t4257, t4293, t49686, t508, t5887, t5920, t651, t6934, t75485, t75494, t75657, t75667, t94);
        let (t75716, t75720, t75727, t75760) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3934(t61014, t75451, t75676, t75714, t1455, t6951, t1464, t6936, t116, t13514, t1459, t1461, t18204, t18211, t18214, t1916, t21881, t22544, t22564, t22565, t2371, t4158, t572, t5795, t5801, t5802, t5805, t670, t6945, t6948);
        let t75792 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3935(t116, t117, t1459, t18190, t18208, t1916, t1918, t22556, t22559, t22568, t2327, t2371, t4162, t4165, t4292, t572, t573, t5883, t5920, t60595, t6941, t75494, t75657, t75716, param_d);
        let t75801 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3936(t22571, t571, t1458, t18178, t18217, t1914, t1921, t4168, t5790, t5808, t60609, t60611, t60616, t60618, t6937, t75727, t75760, t75792);
    (t75716, t75720, t75801)
}
