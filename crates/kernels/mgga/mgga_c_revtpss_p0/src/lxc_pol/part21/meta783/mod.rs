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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta783<F: Float>(t10535: F, t136: F, t2457: F, t4424: F, t10523: F, t14568: F, t2482: F, t2801: F, t4423: F, t879: F, t14606: F, t1568: F, t2722: F, t2723: F, t2782: F, t4503: F, t10661: F, t14602: F, t1558: F, t14523: F, t9285: F, t10073: F, t14496: F, t231: F, t2783: F, t14946: F, t2710: F, t40938: F, t40942: F, t4469: F, t836: F, t14598: F, t14600: F, t2434: F, t10111: F, t22: F, t4518: F, t10871: F, t10952: F, t122: F, t676: F, t72: F, t51306: F, t39698: F, t4494: F, t51375: F, t14509: F, t10069: F, t40921: F, t4496: F, t14537: F, t10532: F, t50511: F, t2797: F, t2645: F, t1559: F, t40927: F, t40945: F, t40948: F, t40952: F, t40954: F, t40956: F, t40958: F, t820: F, t10666: F, t10861: F, t10943: F, t11010: F, t14489: F, t14507: F, t14972: F, t213: F, t234: F, t2724: F, t2754: F, t2765: F, t39576: F, t39581: F, t39586: F, t39590: F, t39595: F, t39602: F, t39606: F, t39610: F, t39649: F, t39652: F, t39662: F, t39668: F, t39673: F, t39678: F, t39731: F, t40267: F, t40271: F, t40273: F, t40278: F, t40282: F, t40294: F, t40914: F, t40918: F, t40922: F, t40924: F, t41073: F, t41092: F, t41095: F, t41098: F, t41102: F, t41105: F, t4366: F, t4474: F, t4504: F, t4514: F, t51184: F, t51256: F, t51260: F, t51263: F, t51264: F, t51269: F, t51272: F, t51277: F, t51327: F, t51360: F, t51387: F, t51390: F, t51396: F, t51403: F, t51431: F, t51456: F, t51479: F, t51484: F, t51515: F, t51552: F, t51589: F, t51598: F, t51600: F, t51604: F, t51610: F, t865: F, t868: F, t887: F, t10504: F, t4533: F, t14481: F, t861: F, t11050: F, t14987: F, t14473: F, t9303: F, t41017: F, t4481: F, t14477: F, t2435: F, t14978: F, t2465: F, t686: F, t14480: F, t252: F, t2828: F, t14482: F, t2444: F, t4534: F, t689: F, t10977: F, t10978: F, t1579: F, t2770: F, t41115: F, t41118: F, t41125: F, t41129: F, t10489: F, t11054: F, t11084: F, t1940: F, t198: F, t207: F, t2403: F, t39989: F, t4343: F, t4541: F, t4542: F, t4556: F, t50106: F, t50114: F, t50115: F, t50151: F, t50190: F, t50216: F, t50250: F, t50276: F, t50853: F, t50857: F, t51218: F, t51253: F, t765: F, t892: F) -> F {
        let (t51615, t51617, t51621, t51623, t51625) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2809::<F>(t10535, t136, t2457, t4424, t10523, t14568, t2482, t2801, t4423, t879, t14606, t1568, t2722);
        let (t51628, t51632, t51635, t51637) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2810::<F>(t2723, t2782, t4503, t51625, t10661, t14602, t1558, t2482, t10535, t14523, t9285, t10073, t14496);
        let t51648 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2811::<F>(t231, t2782, t2783, t51625, t14946, t2710, t9285, t40938, t40942, t51617, t51621, t51623, t51628, t51632, t51635, t51637);
        let (t51653, t51657, t51660, t51668) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2812::<F>(t231, t2782, t2783, t4469, t836, t14598, t14600, t2434, t10111, t22, t4518, t10871, t10952, t122, t1558, t2482, t2722, t676, t72);
        let (t51672, t51676, t51680, t51683, t51685, t51686) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2813::<F>(t231, t2782, t2783, t51306, t22, t39698, t4494, t51375, t10073, t14509, t10069, t40921, t4496);
        let t51690 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2814::<F>(t10073, t14537, t51653, t51657, t51660, t51668, t51672, t51676, t51680, t51683, t51685, t51686);
        let t51713 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2815::<F>(t10532, t14598, t231, t50511, t2782, t2797, t10069, t14537, t1568, t2645, t2783, t1559, t40927, t40945, t40948, t40952, t40954, t40956, t40958, t820);
        let t51723 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2816::<F>(t10666, t10861, t10943, t11010, t14489, t14507, t14972, t213, t234, t2724, t2754, t2765, t39576, t39581, t39586, t39590, t39595, t39602, t39606, t39610, t39649, t39652, t39662, t39668, t39673, t39678, t39731, t40267, t40271, t40273, t40278, t40282, t40294, t40914, t40918, t40922, t40924, t41073, t41092, t41095, t41098, t41102, t41105, t4366, t4474, t4494, t4504, t4514, t51184, t51256, t51260, t51263, t51264, t51269, t51272, t51277, t51306, t51327, t51360, t51387, t51390, t51396, t51403, t51431, t51456, t51479, t51484, t51515, t51552, t51589, t51598, t51600, t51604, t51610, t51615, t51648, t51690, t51713, t820, t865, t868, t887);
        let (t51727, t51729, t51731, t51733, t51739) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2817::<F>(t10504, t136, t2457, t4533, t14481, t2782, t861, t11050, t14987, t14473, t9303, t41017, t4481);
        let (t51742, t51746, t51750, t51756) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2818::<F>(t14477, t2435, t14978, t2465, t686, t72, t14480, t252, t2782, t2828, t10073, t14482);
        let t51762 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2819::<F>(t2444, t4534, t689, t10977, t10978, t1579, t2770, t41115, t41118, t41125, t41129, t4474, t51727, t51729, t51731, t51733, t51739, t51742, t51746, t51750, t51756, t865);
        let t51769 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2820::<F>(t10489, t11054, t11084, t1940, t198, t207, t2403, t39989, t4343, t4541, t4542, t4556, t50106, t50114, t50115, t50151, t50190, t50216, t50250, t50276, t50853, t50857, t51218, t51253, t51723, t51762, t765, t892);
    t51769
}
