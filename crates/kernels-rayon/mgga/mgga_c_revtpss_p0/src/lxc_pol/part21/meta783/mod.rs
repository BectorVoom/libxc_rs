//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta783 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2809;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2810;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2811;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2812;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2813;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2814;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2815;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2816;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2817;
use chunk9::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2818;
use chunk10::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2819;
use chunk11::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2820;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta783(t10535: f64, t136: f64, t2457: f64, t4424: f64, t10523: f64, t14568: f64, t2482: f64, t2801: f64, t4423: f64, t879: f64, t14606: f64, t1568: f64, t2722: f64, t2723: f64, t2782: f64, t4503: f64, t10661: f64, t14602: f64, t1558: f64, t14523: f64, t9285: f64, t10073: f64, t14496: f64, t231: f64, t2783: f64, t14946: f64, t2710: f64, t40938: f64, t40942: f64, t4469: f64, t836: f64, t14598: f64, t14600: f64, t2434: f64, t10111: f64, t22: f64, t4518: f64, t10871: f64, t10952: f64, t122: f64, t676: f64, t72: f64, t51306: f64, t39698: f64, t4494: f64, t51375: f64, t14509: f64, t10069: f64, t40921: f64, t4496: f64, t14537: f64, t10532: f64, t50511: f64, t2797: f64, t2645: f64, t1559: f64, t40927: f64, t40945: f64, t40948: f64, t40952: f64, t40954: f64, t40956: f64, t40958: f64, t820: f64, t10666: f64, t10861: f64, t10943: f64, t11010: f64, t14489: f64, t14507: f64, t14972: f64, t213: f64, t234: f64, t2724: f64, t2754: f64, t2765: f64, t39576: f64, t39581: f64, t39586: f64, t39590: f64, t39595: f64, t39602: f64, t39606: f64, t39610: f64, t39649: f64, t39652: f64, t39662: f64, t39668: f64, t39673: f64, t39678: f64, t39731: f64, t40267: f64, t40271: f64, t40273: f64, t40278: f64, t40282: f64, t40294: f64, t40914: f64, t40918: f64, t40922: f64, t40924: f64, t41073: f64, t41092: f64, t41095: f64, t41098: f64, t41102: f64, t41105: f64, t4366: f64, t4474: f64, t4504: f64, t4514: f64, t51184: f64, t51256: f64, t51260: f64, t51263: f64, t51264: f64, t51269: f64, t51272: f64, t51277: f64, t51327: f64, t51360: f64, t51387: f64, t51390: f64, t51396: f64, t51403: f64, t51431: f64, t51456: f64, t51479: f64, t51484: f64, t51515: f64, t51552: f64, t51589: f64, t51598: f64, t51600: f64, t51604: f64, t51610: f64, t865: f64, t868: f64, t887: f64, t10504: f64, t4533: f64, t14481: f64, t861: f64, t11050: f64, t14987: f64, t14473: f64, t9303: f64, t41017: f64, t4481: f64, t14477: f64, t2435: f64, t14978: f64, t2465: f64, t686: f64, t14480: f64, t252: f64, t2828: f64, t14482: f64, t2444: f64, t4534: f64, t689: f64, t10977: f64, t10978: f64, t1579: f64, t2770: f64, t41115: f64, t41118: f64, t41125: f64, t41129: f64, t10489: f64, t11054: f64, t11084: f64, t1940: f64, t198: f64, t207: f64, t2403: f64, t39989: f64, t4343: f64, t4541: f64, t4542: f64, t4556: f64, t50106: f64, t50114: f64, t50115: f64, t50151: f64, t50190: f64, t50216: f64, t50250: f64, t50276: f64, t50853: f64, t50857: f64, t51218: f64, t51253: f64, t765: f64, t892: f64) -> f64 {
        let (t51615, t51617, t51621, t51623, t51625) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2809(t10535, t136, t2457, t4424, t10523, t14568, t2482, t2801, t4423, t879, t14606, t1568, t2722);
        let (t51628, t51632, t51635, t51637) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2810(t2723, t2782, t4503, t51625, t10661, t14602, t1558, t2482, t10535, t14523, t9285, t10073, t14496);
        let t51648 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2811(t231, t2782, t2783, t51625, t14946, t2710, t9285, t40938, t40942, t51617, t51621, t51623, t51628, t51632, t51635, t51637);
        let (t51653, t51657, t51660, t51668) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2812(t231, t2782, t2783, t4469, t836, t14598, t14600, t2434, t10111, t22, t4518, t10871, t10952, t122, t1558, t2482, t2722, t676, t72);
        let (t51672, t51676, t51680, t51683, t51685, t51686) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2813(t231, t2782, t2783, t51306, t22, t39698, t4494, t51375, t10073, t14509, t10069, t40921, t4496);
        let t51690 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2814(t10073, t14537, t51653, t51657, t51660, t51668, t51672, t51676, t51680, t51683, t51685, t51686);
        let t51713 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2815(t10532, t14598, t231, t50511, t2782, t2797, t10069, t14537, t1568, t2645, t2783, t1559, t40927, t40945, t40948, t40952, t40954, t40956, t40958, t820);
        let t51723 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2816(t10666, t10861, t10943, t11010, t14489, t14507, t14972, t213, t234, t2724, t2754, t2765, t39576, t39581, t39586, t39590, t39595, t39602, t39606, t39610, t39649, t39652, t39662, t39668, t39673, t39678, t39731, t40267, t40271, t40273, t40278, t40282, t40294, t40914, t40918, t40922, t40924, t41073, t41092, t41095, t41098, t41102, t41105, t4366, t4474, t4494, t4504, t4514, t51184, t51256, t51260, t51263, t51264, t51269, t51272, t51277, t51306, t51327, t51360, t51387, t51390, t51396, t51403, t51431, t51456, t51479, t51484, t51515, t51552, t51589, t51598, t51600, t51604, t51610, t51615, t51648, t51690, t51713, t820, t865, t868, t887);
        let (t51727, t51729, t51731, t51733, t51739) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2817(t10504, t136, t2457, t4533, t14481, t2782, t861, t11050, t14987, t14473, t9303, t41017, t4481);
        let (t51742, t51746, t51750, t51756) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2818(t14477, t2435, t14978, t2465, t686, t72, t14480, t252, t2782, t2828, t10073, t14482);
        let t51762 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2819(t2444, t4534, t689, t10977, t10978, t1579, t2770, t41115, t41118, t41125, t41129, t4474, t51727, t51729, t51731, t51733, t51739, t51742, t51746, t51750, t51756, t865);
        let t51769 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2820(t10489, t11054, t11084, t1940, t198, t207, t2403, t39989, t4343, t4541, t4542, t4556, t50106, t50114, t50115, t50151, t50190, t50216, t50250, t50276, t50853, t50857, t51218, t51253, t51723, t51762, t765, t892);
    t51769
}
