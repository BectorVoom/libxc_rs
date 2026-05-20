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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta856<F: Float>(t10355: F, t44: F, t10368: F, t56: F, t10326: F, t10345: F, t10369: F, t10376: F, t13312: F, t13313: F, t13324: F, t1474: F, t1480: F, t2258: F, t2270: F, t2282: F, t4205: F, t4210: F, t46090: F, t48: F, t49889: F, t51959: F, t60: F, t606: F, t614: F, t10321: F, t13335: F, t13340: F, t13400: F, t13405: F, t1494: F, t2251: F, t2252: F, t2291: F, t2312: F, t36: F, t38: F, t4181: F, t4217: F, t4218: F, t4238: F, t60297: F, t627: F, t641: F, t70: F, t85: F, t10318: F, t10327: F, t10380: F, t10407: F, t13334: F, t13406: F, t13409: F, t13414: F, t1470: F, t1471: F, t1486: F, t4182: F, t4187: F, t4188: F, t4191: F, t607: F, t72: F, t10317: F, t10328: F, t10331: F, t10336: F, t13343: F, t13346: F, t13389: F, t2259: F, t2260: F, t2263: F, t4196: F, t608: F, t7719: F, t10356: F, t10389: F, t10398: F, t11231: F, t13368: F, t13371: F, t13378: F, t13381: F, t1469: F, t2299: F, t2306: F, t4186: F, t4227: F, t4232: F, t46001: F, t46014: F, t633: F, t637: F, t10381: F, t13392: F, t13393: F, t13396: F, t13397: F, t1487: F, t2292: F, t53459: F, t53464: F, t54450: F, t628: F, t71: F, t77: F, t10298: F, t10309: F, t10310: F, t10410: F, t13269: F, t13283: F, t13420: F, t1497: F, t2242: F, t2247: F, t2315: F, t4173: F, t4178: F, t4241: F, t45955: F, t45963: F, t45972: F, t60248: F, t603: F, t644: F, t5: F, t10301: F, t10313: F, t13272: F, t13286: F, t13289: F, t2248: F, t45931: F, t45933: F, t45941: F, t45944: F, t45952: F, t45958: F, t60214: F, t60215: F, t60216: F, t60217: F, t60218: F, t60221: F, t60224: F, t91: F, t117: F, t10259: F, t93: F, t10416: F, t1312: F, t13426: F, t13435: F, t13440: F, t13514: F, t1518: F, t18227: F, t2322: F, t2371: F, t4248: F, t4292: F, t46126: F, t49686: F, t49693: F, t49830: F, t49851: F, t5523: F, t60206: F, t670: F, t10426: F, t1310: F, t13207: F, t13216: F, t13425: F, t13517: F, t13537: F, t13544: F, t1453: F, t1843: F, t1911: F, t2372: F, t4254: F, t4297: F, t508: F, t569: F, t651: F, t49834: F, t60183: F, t60213: F, t116: F, t13232: F, t13240: F, t13244: F, t13247: F, t1459: F, t1461: F, t18190: F, t18204: F, t18207: F, t18208: F, t18211: F, t18214: F, t1916: F, t1918: F, t2327: F, t4158: F, t4162: F, t4165: F, t572: F, t573: F, t5795: F, t5801: F, t5802: F, t5805: F, param_d: F, t1913: F, t4168: F, t18217: F, t571: F, t1921: F, t4153: F, t1464: F, t5789: F, t18177: F, t575: F, t13226: F, t13250: F, t1456: F, t1458: F, t18178: F, t1914: F, t3: F, t39397: F, t39399: F, t39401: F, t39403: F, t4154: F, t47730: F, t5790: F, t5808: F) -> F {
        let t60330 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3247::<F>(t10355, t44, t10368, t56, t10326, t10345, t10369, t10376, t13312, t13313, t13324, t1474, t1480, t2258, t2270, t2282, t4205, t4210, t46090, t48, t49889, t51959, t60, t606, t614);
        let t60360 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3248::<F>(t10321, t13335, t13340, t13400, t13405, t1494, t2251, t2252, t2291, t2312, t36, t38, t4181, t4217, t4218, t4238, t49889, t60297, t60330, t627, t641, t70, t85);
        let t60391 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3249::<F>(t10318, t10327, t10380, t10407, t13334, t13406, t13409, t13414, t1470, t1471, t1486, t2291, t2312, t4182, t4187, t4188, t4191, t606, t607, t641, t72, t85);
        let t60417 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3250::<F>(t10317, t10328, t10331, t10336, t13343, t13346, t13389, t1494, t2258, t2259, t2260, t2263, t2312, t4196, t4217, t4238, t608, t641, t7719, t85);
        let t60479 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3251::<F>(t10326, t10356, t10389, t10398, t11231, t13312, t13368, t13371, t13378, t13381, t1469, t2251, t2258, t2299, t2306, t4186, t4227, t4232, t46001, t46014, t49889, t606, t633, t637);
        let t60483 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3252::<F>(t10381, t10407, t13389, t13392, t13393, t13396, t13397, t1487, t1494, t2292, t4238, t53459, t53464, t54450, t60479, t627, t628, t641, t70, t71, t77, t85);
        let t60496 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3253::<F>(t10298, t10309, t10310, t10410, t13269, t13283, t13420, t1497, t2242, t2247, t2315, t4173, t4178, t4241, t45955, t45963, t45972, t60248, t603, t60360, t60391, t60417, t60483, t644);
        let t60498 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3254::<F>(t5, t10301, t10309, t10310, t10313, t13272, t13286, t13289, t13420, t2247, t2248, t2315, t4178, t4241, t45931, t45933, t45941, t45944, t45952, t45958, t60214, t60215, t60216, t60217, t60218, t60221, t60224, t60496, t644, t91);
        let (t60499, t60556) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3255::<F>(t117, t60498, t10259, t93, t10416, t1312, t13426, t13435, t13440, t13514, t1518, t18227, t2322, t2371, t4248, t4292, t46126, t49686, t49693, t49830, t49851, t5523, t60206, t670);
        let t60558 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3256::<F>(t10259, t10416, t10426, t1310, t13207, t13216, t13425, t13426, t13517, t13537, t13544, t1453, t1518, t18227, t1843, t1911, t2322, t2372, t4248, t4254, t4297, t508, t569, t60499, t60556, t651);
        let (t60560, t60599) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3257::<F>(t49834, t60183, t60213, t60558, t1518, t670, t10259, t116, t117, t13232, t13240, t13244, t13247, t13514, t1459, t1461, t18190, t18204, t18207, t18208, t18211, t18214, t1916, t1918, t2327, t2371, t4158, t4162, t4165, t4292, t49830, t572, t573, t5795, t5801, t5802, t5805, param_d);
        let tv4rho41 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3258::<F>(t1913, t4168, t18217, t571, t1921, t4153, t1464, t5789, t18177, t575, t13226, t13250, t1456, t1458, t18178, t1914, t3, t39397, t39399, t39401, t39403, t4154, t47730, t5790, t5808, t60560, t60599);
    tv4rho41
}
