//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta856 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3247;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3248;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3249;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3250;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3251;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3252;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3253;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3254;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3255;
use chunk9::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3256;
use chunk10::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3257;
use chunk11::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3258;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta856(t10355: f64, t44: f64, t10368: f64, t56: f64, t10326: f64, t10345: f64, t10369: f64, t10376: f64, t13312: f64, t13313: f64, t13324: f64, t1474: f64, t1480: f64, t2258: f64, t2270: f64, t2282: f64, t4205: f64, t4210: f64, t46090: f64, t48: f64, t49889: f64, t51959: f64, t60: f64, t606: f64, t614: f64, t10321: f64, t13335: f64, t13340: f64, t13400: f64, t13405: f64, t1494: f64, t2251: f64, t2252: f64, t2291: f64, t2312: f64, t36: f64, t38: f64, t4181: f64, t4217: f64, t4218: f64, t4238: f64, t60297: f64, t627: f64, t641: f64, t70: f64, t85: f64, t10318: f64, t10327: f64, t10380: f64, t10407: f64, t13334: f64, t13406: f64, t13409: f64, t13414: f64, t1470: f64, t1471: f64, t1486: f64, t4182: f64, t4187: f64, t4188: f64, t4191: f64, t607: f64, t72: f64, t10317: f64, t10328: f64, t10331: f64, t10336: f64, t13343: f64, t13346: f64, t13389: f64, t2259: f64, t2260: f64, t2263: f64, t4196: f64, t608: f64, t7719: f64, t10356: f64, t10389: f64, t10398: f64, t11231: f64, t13368: f64, t13371: f64, t13378: f64, t13381: f64, t1469: f64, t2299: f64, t2306: f64, t4186: f64, t4227: f64, t4232: f64, t46001: f64, t46014: f64, t633: f64, t637: f64, t10381: f64, t13392: f64, t13393: f64, t13396: f64, t13397: f64, t1487: f64, t2292: f64, t53459: f64, t53464: f64, t54450: f64, t628: f64, t71: f64, t77: f64, t10298: f64, t10309: f64, t10310: f64, t10410: f64, t13269: f64, t13283: f64, t13420: f64, t1497: f64, t2242: f64, t2247: f64, t2315: f64, t4173: f64, t4178: f64, t4241: f64, t45955: f64, t45963: f64, t45972: f64, t60248: f64, t603: f64, t644: f64, t5: f64, t10301: f64, t10313: f64, t13272: f64, t13286: f64, t13289: f64, t2248: f64, t45931: f64, t45933: f64, t45941: f64, t45944: f64, t45952: f64, t45958: f64, t60214: f64, t60215: f64, t60216: f64, t60217: f64, t60218: f64, t60221: f64, t60224: f64, t91: f64, t117: f64, t10259: f64, t93: f64, t10416: f64, t1312: f64, t13426: f64, t13435: f64, t13440: f64, t13514: f64, t1518: f64, t18227: f64, t2322: f64, t2371: f64, t4248: f64, t4292: f64, t46126: f64, t49686: f64, t49693: f64, t49830: f64, t49851: f64, t5523: f64, t60206: f64, t670: f64, t10426: f64, t1310: f64, t13207: f64, t13216: f64, t13425: f64, t13517: f64, t13537: f64, t13544: f64, t1453: f64, t1843: f64, t1911: f64, t2372: f64, t4254: f64, t4297: f64, t508: f64, t569: f64, t651: f64, t49834: f64, t60183: f64, t60213: f64, t116: f64, t13232: f64, t13240: f64, t13244: f64, t13247: f64, t1459: f64, t1461: f64, t18190: f64, t18204: f64, t18207: f64, t18208: f64, t18211: f64, t18214: f64, t1916: f64, t1918: f64, t2327: f64, t4158: f64, t4162: f64, t4165: f64, t572: f64, t573: f64, t5795: f64, t5801: f64, t5802: f64, t5805: f64, param_d: f64, t1913: f64, t4168: f64, t18217: f64, t571: f64, t1921: f64, t4153: f64, t1464: f64, t5789: f64, t18177: f64, t575: f64, t13226: f64, t13250: f64, t1456: f64, t1458: f64, t18178: f64, t1914: f64, t3: f64, t39397: f64, t39399: f64, t39401: f64, t39403: f64, t4154: f64, t47730: f64, t5790: f64, t5808: f64) -> f64 {
        let t60330 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3247(t10355, t44, t10368, t56, t10326, t10345, t10369, t10376, t13312, t13313, t13324, t1474, t1480, t2258, t2270, t2282, t4205, t4210, t46090, t48, t49889, t51959, t60, t606, t614);
        let t60360 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3248(t10321, t13335, t13340, t13400, t13405, t1494, t2251, t2252, t2291, t2312, t36, t38, t4181, t4217, t4218, t4238, t49889, t60297, t60330, t627, t641, t70, t85);
        let t60391 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3249(t10318, t10327, t10380, t10407, t13334, t13406, t13409, t13414, t1470, t1471, t1486, t2291, t2312, t4182, t4187, t4188, t4191, t606, t607, t641, t72, t85);
        let t60417 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3250(t10317, t10328, t10331, t10336, t13343, t13346, t13389, t1494, t2258, t2259, t2260, t2263, t2312, t4196, t4217, t4238, t608, t641, t7719, t85);
        let t60479 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3251(t10326, t10356, t10389, t10398, t11231, t13312, t13368, t13371, t13378, t13381, t1469, t2251, t2258, t2299, t2306, t4186, t4227, t4232, t46001, t46014, t49889, t606, t633, t637);
        let t60483 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3252(t10381, t10407, t13389, t13392, t13393, t13396, t13397, t1487, t1494, t2292, t4238, t53459, t53464, t54450, t60479, t627, t628, t641, t70, t71, t77, t85);
        let t60496 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3253(t10298, t10309, t10310, t10410, t13269, t13283, t13420, t1497, t2242, t2247, t2315, t4173, t4178, t4241, t45955, t45963, t45972, t60248, t603, t60360, t60391, t60417, t60483, t644);
        let t60498 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3254(t5, t10301, t10309, t10310, t10313, t13272, t13286, t13289, t13420, t2247, t2248, t2315, t4178, t4241, t45931, t45933, t45941, t45944, t45952, t45958, t60214, t60215, t60216, t60217, t60218, t60221, t60224, t60496, t644, t91);
        let (t60499, t60556) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3255(t117, t60498, t10259, t93, t10416, t1312, t13426, t13435, t13440, t13514, t1518, t18227, t2322, t2371, t4248, t4292, t46126, t49686, t49693, t49830, t49851, t5523, t60206, t670);
        let t60558 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3256(t10259, t10416, t10426, t1310, t13207, t13216, t13425, t13426, t13517, t13537, t13544, t1453, t1518, t18227, t1843, t1911, t2322, t2372, t4248, t4254, t4297, t508, t569, t60499, t60556, t651);
        let (t60560, t60599) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3257(t49834, t60183, t60213, t60558, t1518, t670, t10259, t116, t117, t13232, t13240, t13244, t13247, t13514, t1459, t1461, t18190, t18204, t18207, t18208, t18211, t18214, t1916, t1918, t2327, t2371, t4158, t4162, t4165, t4292, t49830, t572, t573, t5795, t5801, t5802, t5805, param_d);
        let tv4rho41 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3258(t1913, t4168, t18217, t571, t1921, t4153, t1464, t5789, t18177, t575, t13226, t13250, t1456, t1458, t18178, t1914, t3, t39397, t39399, t39401, t39403, t4154, t47730, t5790, t5808, t60560, t60599);
    tv4rho41
}
