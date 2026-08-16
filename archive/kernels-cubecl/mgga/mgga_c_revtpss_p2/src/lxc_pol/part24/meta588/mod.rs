//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta588 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1836;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1837;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1838;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1839;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1840;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1841;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1842;
use chunk7::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1843;
use chunk8::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1844;
use chunk9::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1845;
use chunk10::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1846;
use chunk11::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1847;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta588<F: Float>(t22858: F, t22954: F, t47417: F, t49327: F, t49354: F, t49361: F, t5767: F, t820: F, t86575: F, t86582: F, t86586: F, t86597: F, t86604: F, t86608: F, t1437: F, t22953: F, t4003: F, t47442: F, t47454: F, t49432: F, t5735: F, t5745: F, t75274: F, t86634: F, t86639: F, t86643: F, t86647: F, t86654: F, t92064: F, t6895: F, t14171: F, t1424: F, t1427: F, t1883: F, t1904: F, t21981: F, t22005: F, t22390: F, t22863: F, t22971: F, t22975: F, t23037: F, t46362: F, t46412: F, t47591: F, t47601: F, t47967: F, t47971: F, t48005: F, t49172: F, t49178: F, t49203: F, t49210: F, t5715: F, t5755: F, t6844: F, t6862: F, t6896: F, t6918: F, t6919: F, t74807: F, t74838: F, t74849: F, t74873: F, t74945: F, t74990: F, t75074: F, t75092: F, t75113: F, t75119: F, t75123: F, t75128: F, t75228: F, t86350: F, t86354: F, t86358: F, t86401: F, t86411: F, t86415: F, t86455: F, t86506: F, t86682: F, t86699: F, t86701: F, t86712: F, t92070: F, t92317: F, t92347: F, t92378: F, t9657: F, t1343: F, t1450: F, t1868: F, t198: F, t21937: F, t39419: F, t39422: F, t4139: F, t46292: F, t46297: F, t46303: F, t532: F, t5536: F, t6816: F, t6836: F, t86731: F, t86839: F, t91826: F, t91952: F, t91953: F, t91954: F, t91955: F, t92229: F, t92248: F, t92267: F, t22466: F, t22852: F, t39483: F, t39520: F, t39528: F, t39531: F, t46963: F, t46970: F, t46972: F, t5532: F, t91956: F, t91958: F, t91959: F, t91960: F, t91961: F, t91962: F, t1907: F, t22483: F, t22809: F, t22813: F, t30122: F, t39747: F, t39750: F, t39756: F, t39760: F, t46980: F, t46988: F, t46992: F, t46996: F, t46998: F, t47000: F, t47003: F, t91963: F, t39773: F, t39783: F, t39786: F, t39791: F, t39795: F, t39799: F, t39807: F, t39813: F, t47014: F, t47017: F, t47020: F, t47059: F, t91966: F, t91968: F, t91969: F, t47067: F, t47070: F, t47072: F, t47074: F, t47076: F, t91970: F, t91974: F, t91975: F, t91976: F, t91977: F, t91978: F, t91979: F, t91980: F, t91982: F, t91983: F, t6922: F, t39989: F, t4147: F, t47084: F, t47086: F, t5541: F, t6781: F, t73499: F, t86819: F, t86825: F, t86828: F, t91984: F, t91985: F, t92013: F, t92014: F, t92015: F, t92016: F, t40067: F, t40072: F, t47088: F, t47092: F, t47096: F, t47098: F, t47109: F, t47116: F, t47118: F, t47672: F, t92019: F, t92020: F, t92021: F, t92022: F, t3828: F, t40076: F, t40079: F, t47122: F, t47124: F, t47131: F, t47138: F, t47140: F, t47142: F, t47152: F, t91875: F, t92024: F, t92026: F, t92027: F, t92028: F, t92029: F, t118: F, t1312: F, t1502: F, t1518: F, t18245: F, t1843: F, t1847: F, t1911: F, t22633: F, t22747: F, t22758: F, t23094: F, t25043: F, t25045: F, t30138: F, t4248: F, t508: F, t511: F, t569: F, t5877: F, t5920: F, t5921: F, t651: F, t6765: F, t6773: F, t6934: F, t75941: F, t7889: F, t87051: F, t87064: F, t87227: F, t87237: F, t89771: F, t91789: F, t93: F, t94: F) -> F {
        let t92394 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1836::<F>(t22858, t22954, t47417, t49327, t49354, t49361, t5767, t820, t86575, t86582, t86586, t86597, t86604, t86608);
        let t92409 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1837::<F>(t1437, t22953, t4003, t47442, t47454, t49432, t5735, t5745, t75274, t820, t86634, t86639, t86643, t86647, t86654, t92064);
        let t92434 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1838::<F>(t6895, t14171, t1424, t1427, t1437, t1883, t1904, t21981, t22005, t22390, t22863, t22971, t22975, t23037, t46362, t46412, t47591, t47601, t47967, t47971, t48005, t49172, t49178, t49203, t49210, t5715, t5745, t5755, t6844, t6862, t6896, t6918, t6919, t74807, t74838, t74849, t74873, t74945, t74990, t75074, t75092, t75113, t75119, t75123, t75128, t75228, t820, t86350, t86354, t86358, t86401, t86411, t86415, t86455, t86506, t86682, t86699, t86701, t86712, t92070, t92317, t92347, t92378, t92394, t92409, t9657);
        let t92446 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1839::<F>(t1343, t1450, t1868, t198, t21937, t39419, t39422, t4139, t46292, t46297, t46303, t532, t5536, t6816, t6836, t86731, t86839, t91826, t91952, t91953, t91954, t91955, t92229, t92248, t92267, t92434);
        let t92453 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1840::<F>(t22466, t22852, t39483, t39520, t39528, t39531, t4139, t46963, t46970, t46972, t5532, t5536, t6816, t91956, t91958, t91959, t91960, t91961, t91962);
        let t92465 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1841::<F>(t1450, t1907, t198, t22483, t22809, t22813, t30122, t39747, t39750, t39756, t39760, t4139, t46980, t46988, t46992, t46996, t46998, t47000, t47003, t5532, t91963);
        let t92466 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1842::<F>(t39773, t39783, t39786, t39791, t39795, t39799, t39807, t39813, t47014, t47017, t47020, t47059, t91966, t91968, t91969);
        let t92469 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1843::<F>(t47067, t47070, t47072, t47074, t47076, t91970, t91974, t91975, t91976, t91977, t91978, t91979, t91980, t91982, t91983);
        let t92490 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1844::<F>(t6922, t1868, t1907, t198, t21937, t39989, t4139, t4147, t47084, t47086, t532, t5536, t5541, t6781, t6816, t73499, t86819, t86825, t86828, t91984, t91985, t92013, t92014, t92015, t92016);
        let t92500 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1845::<F>(t6781, t198, t22466, t40067, t40072, t47088, t47092, t47096, t47098, t47109, t47116, t47118, t47672, t532, t5536, t6836, t92019, t92020, t92021, t92022);
        let t92504 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1846::<F>(t198, t3828, t40076, t40079, t47122, t47124, t47131, t47138, t47140, t47142, t47152, t91875, t92024, t92026, t92027, t92028, t92029);
        let t92516 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1847::<F>(t118, t1312, t1502, t1518, t18245, t1843, t1847, t1911, t22633, t22747, t22758, t23094, t25043, t25045, t30138, t4248, t508, t511, t569, t5877, t5920, t5921, t651, t6765, t6773, t6934, t75941, t7889, t87051, t87064, t87227, t87237, t89771, t91789, t92446, t92453, t92465, t92466, t92469, t92490, t92500, t92504, t93, t94);
    t92516
}
