//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta456 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1319;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1320;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1321;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1322;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1323;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1324;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1325;
use chunk7::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1326;
use chunk8::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1327;
use chunk9::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1328;
use chunk10::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1329;
use chunk11::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1330;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta456(t1484: f64, t5611: f64, t13222: f64, t13350: f64, t1510: f64, t16891: f64, t20947: f64, t20972: f64, t20993: f64, t210: f64, t2571: f64, t2643: f64, t46876: f64, t5544: f64, t5567: f64, t58723: f64, t58744: f64, t67880: f64, t67882: f64, t67884: f64, t67920: f64, t67937: f64, t9559: f64, t9646: f64, t232: f64, t76001: f64, t2632: f64, t76085: f64, t13283: f64, t20963: f64, t20969: f64, t20981: f64, t2630: f64, t2645: f64, t41096: f64, t4167: f64, t4178: f64, t5527: f64, t58809: f64, t67607: f64, t67644: f64, t67976: f64, t67978: f64, t67980: f64, t76090: f64, t817: f64, t819: f64, t820: f64, t843: f64, t9607: f64, t9974: f64, t119: f64, t16872: f64, t20800: f64, t20904: f64, t20949: f64, t2701: f64, t41139: f64, t41349: f64, t4172: f64, t46957: f64, t47047: f64, t5614: f64, t5619: f64, t68021: f64, t75978: f64, t76002: f64, t76074: f64, t76086: f64, t787: f64, t13005: f64, t16771: f64, t214: f64, t221: f64, t41155: f64, t41161: f64, t41185: f64, t41200: f64, t4127: f64, t4128: f64, t46764: f64, t46772: f64, t46790: f64, t68073: f64, t68110: f64, t76056: f64, t76063: f64, t41209: f64, t41212: f64, t46806: f64, t59195: f64, t59204: f64, t59206: f64, t59218: f64, t59221: f64, t59224: f64, t68116: f64, t68118: f64, t68122: f64, t68131: f64, t225: f64, t13228: f64, t1512: f64, t20953: f64, t237: f64, t249: f64, t59259: f64, t59263: f64, t59276: f64, t59288: f64, t67872: f64, t68148: f64, t68195: f64, t68197: f64, t68199: f64, t68201: f64, t76132: f64, t76167: f64, t76193: f64, t76227: f64, t10080: f64, t1499: f64, t16673: f64, t17027: f64, t20857: f64, t20858: f64, t21014: f64, t226: f64, t235: f64, t255: f64, t40932: f64, t4166: f64, t46524: f64, t5585: f64, t5612: f64, t5617: f64, t5653: f64, t59355: f64, t812: f64, t13416: f64, t1525: f64, t20853: f64, t20854: f64, t20861: f64, t20870: f64, t20871: f64, t20876: f64, t20937: f64, t2728: f64, t4281: f64, t4295: f64, t5575: f64, t5645: f64, t5655: f64, t67392: f64, t5636: f64, t13397: f64, t1492: f64, t1523: f64, t1528: f64, t16758: f64, t16815: f64, t16830: f64, t17034: f64, t17052: f64, t17090: f64, t17092: f64, t20806: f64, t20862: f64, t20867: f64, t20873: f64, t20986: f64, t21013: f64, t21025: f64, t21028: f64, t21034: f64, t21050: f64, t259: f64, t40890: f64, t4147: f64, t4268: f64, t4291: f64, t5637: f64, t5648: f64, t5651: f64, t5658: f64, t67305: f64, t67339: f64, t67344: f64, t67405: f64, t67429: f64, t67441: f64, t68246: f64, t855: f64, t858: f64, t860: f64, t5657: f64, t10110: f64, t1519: f64, t1527: f64, t20936: f64, t21033: f64, t21054: f64, t218: f64, t252: f64, t2718: f64, t5558: f64, t5631: f64, t68322: f64, t193: f64, t202: f64, t2522: f64, t39593: f64, t41254: f64, t4310: f64, t67112: f64, t75950: f64, t75951: f64, t75952: f64, t76017: f64, t76018: f64, t76020: f64, t76024: f64, t76025: f64, t766: f64, t870: f64, t16606: f64, t16625: f64, t2378: f64, t39658: f64, t41258: f64, t41262: f64, t4314: f64, t68371: f64, t76026: f64, t76027: f64, t76030: f64, t76031: f64, t76034: f64, t76035: f64, t76037: f64) -> (f64, f64) {
        let (t76250, t76259) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1319(t1484, t5611, t13222, t13350, t1510, t16891, t20947, t20972, t20993, t210, t2571, t2643, t46876, t5544, t5567, t58723, t58744, t67880, t67882, t67884, t67920, t67937, t9559, t9646);
        let (t76274, t76290, t76295) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1320(t232, t76001, t2632, t76085, t13283, t1510, t20963, t20969, t20981, t2630, t2643, t2645, t41096, t4167, t4178, t5527, t5544, t58809, t67607, t67644, t67976, t67978, t67980, t76090, t817, t819, t820, t843, t9607, t9974);
        let (t76327, t76333) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1321(t232, t76085, t119, t1484, t16872, t20800, t20904, t20949, t210, t2630, t2701, t41139, t41349, t4172, t46957, t47047, t5614, t5619, t68021, t75978, t76002, t76074, t76086, t787, t817, t819, t820, t843);
        let t76359 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1322(t13005, t16771, t20800, t210, t214, t221, t2571, t41155, t41161, t41185, t41200, t4127, t4128, t46764, t46772, t46790, t5544, t68073, t68110, t75978, t76056, t76063, t787);
        let t76371 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1323(t41209, t41212, t46806, t59195, t59204, t59206, t59218, t59221, t59224, t68116, t68118, t68122, t68131);
        let (t76372, t76373, t76394) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1324(t76359, t76371, t225, t13222, t13228, t1512, t20953, t237, t249, t4167, t4178, t59259, t59263, t59276, t59288, t67872, t68148, t68195, t68197, t68199, t68201, t76250);
        let (t76397, t76414) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1325(t76132, t76167, t76193, t76227, t76259, t76295, t76333, t76394, t10080, t1499, t16673, t17027, t20857, t20858, t21014, t226, t235, t255, t40932, t4166, t46524, t5585, t5612, t5617, t5653, t59355, t76086, t76090, t76373, t812);
        let t76467 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1326(t13228, t13416, t1525, t16673, t20853, t20854, t20861, t20870, t20871, t20876, t20937, t2728, t4166, t4281, t4295, t5575, t5645, t5655, t67392, t76290, t812);
        let t76497 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1327(t5636, t13397, t1492, t1510, t1523, t1528, t16673, t16758, t16815, t16830, t17034, t17052, t17090, t17092, t20806, t20862, t20867, t20873, t20986, t21013, t21025, t21028, t21034, t21050, t259, t2728, t40890, t4147, t4166, t4268, t4281, t4291, t5612, t5637, t5648, t5651, t5658, t67305, t67339, t67344, t67392, t67405, t67429, t67441, t68246, t76002, t76074, t76274, t76327, t76414, t76467, t812, t855, t858, t860);
        let t76532 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1328(t5657, t10110, t1519, t1527, t1528, t17052, t17090, t20936, t21033, t21050, t21054, t218, t252, t259, t2718, t4147, t4268, t5558, t5631, t5636, t5637, t5658, t68322, t76372, t76397, t855);
        let t76543 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1329(t1484, t193, t202, t20800, t2522, t39593, t41254, t4310, t67112, t75950, t75951, t75952, t75978, t76017, t76018, t76020, t76024, t76025, t76497, t76532, t766, t870);
        let t76556 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1330(t16606, t16625, t193, t2378, t2522, t39658, t41258, t41262, t4314, t5527, t5544, t68371, t76026, t76027, t76030, t76031, t76034, t76035, t76037, t76063);
    (t76543, t76556)
}
