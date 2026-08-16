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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta588(t22858: f64, t22954: f64, t47417: f64, t49327: f64, t49354: f64, t49361: f64, t5767: f64, t820: f64, t86575: f64, t86582: f64, t86586: f64, t86597: f64, t86604: f64, t86608: f64, t1437: f64, t22953: f64, t4003: f64, t47442: f64, t47454: f64, t49432: f64, t5735: f64, t5745: f64, t75274: f64, t86634: f64, t86639: f64, t86643: f64, t86647: f64, t86654: f64, t92064: f64, t6895: f64, t14171: f64, t1424: f64, t1427: f64, t1883: f64, t1904: f64, t21981: f64, t22005: f64, t22390: f64, t22863: f64, t22971: f64, t22975: f64, t23037: f64, t46362: f64, t46412: f64, t47591: f64, t47601: f64, t47967: f64, t47971: f64, t48005: f64, t49172: f64, t49178: f64, t49203: f64, t49210: f64, t5715: f64, t5755: f64, t6844: f64, t6862: f64, t6896: f64, t6918: f64, t6919: f64, t74807: f64, t74838: f64, t74849: f64, t74873: f64, t74945: f64, t74990: f64, t75074: f64, t75092: f64, t75113: f64, t75119: f64, t75123: f64, t75128: f64, t75228: f64, t86350: f64, t86354: f64, t86358: f64, t86401: f64, t86411: f64, t86415: f64, t86455: f64, t86506: f64, t86682: f64, t86699: f64, t86701: f64, t86712: f64, t92070: f64, t92317: f64, t92347: f64, t92378: f64, t9657: f64, t1343: f64, t1450: f64, t1868: f64, t198: f64, t21937: f64, t39419: f64, t39422: f64, t4139: f64, t46292: f64, t46297: f64, t46303: f64, t532: f64, t5536: f64, t6816: f64, t6836: f64, t86731: f64, t86839: f64, t91826: f64, t91952: f64, t91953: f64, t91954: f64, t91955: f64, t92229: f64, t92248: f64, t92267: f64, t22466: f64, t22852: f64, t39483: f64, t39520: f64, t39528: f64, t39531: f64, t46963: f64, t46970: f64, t46972: f64, t5532: f64, t91956: f64, t91958: f64, t91959: f64, t91960: f64, t91961: f64, t91962: f64, t1907: f64, t22483: f64, t22809: f64, t22813: f64, t30122: f64, t39747: f64, t39750: f64, t39756: f64, t39760: f64, t46980: f64, t46988: f64, t46992: f64, t46996: f64, t46998: f64, t47000: f64, t47003: f64, t91963: f64, t39773: f64, t39783: f64, t39786: f64, t39791: f64, t39795: f64, t39799: f64, t39807: f64, t39813: f64, t47014: f64, t47017: f64, t47020: f64, t47059: f64, t91966: f64, t91968: f64, t91969: f64, t47067: f64, t47070: f64, t47072: f64, t47074: f64, t47076: f64, t91970: f64, t91974: f64, t91975: f64, t91976: f64, t91977: f64, t91978: f64, t91979: f64, t91980: f64, t91982: f64, t91983: f64, t6922: f64, t39989: f64, t4147: f64, t47084: f64, t47086: f64, t5541: f64, t6781: f64, t73499: f64, t86819: f64, t86825: f64, t86828: f64, t91984: f64, t91985: f64, t92013: f64, t92014: f64, t92015: f64, t92016: f64, t40067: f64, t40072: f64, t47088: f64, t47092: f64, t47096: f64, t47098: f64, t47109: f64, t47116: f64, t47118: f64, t47672: f64, t92019: f64, t92020: f64, t92021: f64, t92022: f64, t3828: f64, t40076: f64, t40079: f64, t47122: f64, t47124: f64, t47131: f64, t47138: f64, t47140: f64, t47142: f64, t47152: f64, t91875: f64, t92024: f64, t92026: f64, t92027: f64, t92028: f64, t92029: f64, t118: f64, t1312: f64, t1502: f64, t1518: f64, t18245: f64, t1843: f64, t1847: f64, t1911: f64, t22633: f64, t22747: f64, t22758: f64, t23094: f64, t25043: f64, t25045: f64, t30138: f64, t4248: f64, t508: f64, t511: f64, t569: f64, t5877: f64, t5920: f64, t5921: f64, t651: f64, t6765: f64, t6773: f64, t6934: f64, t75941: f64, t7889: f64, t87051: f64, t87064: f64, t87227: f64, t87237: f64, t89771: f64, t91789: f64, t93: f64, t94: f64) -> f64 {
        let t92394 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1836(t22858, t22954, t47417, t49327, t49354, t49361, t5767, t820, t86575, t86582, t86586, t86597, t86604, t86608);
        let t92409 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1837(t1437, t22953, t4003, t47442, t47454, t49432, t5735, t5745, t75274, t820, t86634, t86639, t86643, t86647, t86654, t92064);
        let t92434 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1838(t6895, t14171, t1424, t1427, t1437, t1883, t1904, t21981, t22005, t22390, t22863, t22971, t22975, t23037, t46362, t46412, t47591, t47601, t47967, t47971, t48005, t49172, t49178, t49203, t49210, t5715, t5745, t5755, t6844, t6862, t6896, t6918, t6919, t74807, t74838, t74849, t74873, t74945, t74990, t75074, t75092, t75113, t75119, t75123, t75128, t75228, t820, t86350, t86354, t86358, t86401, t86411, t86415, t86455, t86506, t86682, t86699, t86701, t86712, t92070, t92317, t92347, t92378, t92394, t92409, t9657);
        let t92446 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1839(t1343, t1450, t1868, t198, t21937, t39419, t39422, t4139, t46292, t46297, t46303, t532, t5536, t6816, t6836, t86731, t86839, t91826, t91952, t91953, t91954, t91955, t92229, t92248, t92267, t92434);
        let t92453 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1840(t22466, t22852, t39483, t39520, t39528, t39531, t4139, t46963, t46970, t46972, t5532, t5536, t6816, t91956, t91958, t91959, t91960, t91961, t91962);
        let t92465 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1841(t1450, t1907, t198, t22483, t22809, t22813, t30122, t39747, t39750, t39756, t39760, t4139, t46980, t46988, t46992, t46996, t46998, t47000, t47003, t5532, t91963);
        let t92466 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1842(t39773, t39783, t39786, t39791, t39795, t39799, t39807, t39813, t47014, t47017, t47020, t47059, t91966, t91968, t91969);
        let t92469 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1843(t47067, t47070, t47072, t47074, t47076, t91970, t91974, t91975, t91976, t91977, t91978, t91979, t91980, t91982, t91983);
        let t92490 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1844(t6922, t1868, t1907, t198, t21937, t39989, t4139, t4147, t47084, t47086, t532, t5536, t5541, t6781, t6816, t73499, t86819, t86825, t86828, t91984, t91985, t92013, t92014, t92015, t92016);
        let t92500 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1845(t6781, t198, t22466, t40067, t40072, t47088, t47092, t47096, t47098, t47109, t47116, t47118, t47672, t532, t5536, t6836, t92019, t92020, t92021, t92022);
        let t92504 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1846(t198, t3828, t40076, t40079, t47122, t47124, t47131, t47138, t47140, t47142, t47152, t91875, t92024, t92026, t92027, t92028, t92029);
        let t92516 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1847(t118, t1312, t1502, t1518, t18245, t1843, t1847, t1911, t22633, t22747, t22758, t23094, t25043, t25045, t30138, t4248, t508, t511, t569, t5877, t5920, t5921, t651, t6765, t6773, t6934, t75941, t7889, t87051, t87064, t87227, t87237, t89771, t91789, t92446, t92453, t92465, t92466, t92469, t92490, t92500, t92504, t93, t94);
    t92516
}
