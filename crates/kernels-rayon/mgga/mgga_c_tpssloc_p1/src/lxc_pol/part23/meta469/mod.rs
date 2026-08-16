//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta469 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1386;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1387;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1388;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1389;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1390;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1391;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1392;
use chunk7::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1393;
use chunk8::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1394;
use chunk9::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1395;
use chunk10::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1396;
use chunk11::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1397;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta469(t1021: f64, t10403: f64, t1041: f64, t10480: f64, t10883: f64, t10970: f64, t1409: f64, t17712: f64, t21405: f64, t21532: f64, t248: f64, t3071: f64, t3146: f64, t360: f64, t42358: f64, t4582: f64, t48670: f64, t48674: f64, t49934: f64, t50193: f64, t5878: f64, t61782: f64, t62079: f64, t62840: f64, t70100: f64, t70239: f64, t70346: f64, t70351: f64, t70363: f64, t70389: f64, t70404: f64, t75847: f64, t76581: f64, t76740: f64, t973: f64, t974: f64, t1044: f64, t14211: f64, t14508: f64, t14511: f64, t1616: f64, t21138: f64, t21487: f64, t21503: f64, t21597: f64, t21603: f64, t21609: f64, t3070: f64, t3130: f64, t3131: f64, t3151: f64, t42444: f64, t4641: f64, t4644: f64, t5685: f64, t5873: f64, t62137: f64, t62148: f64, t62177: f64, t62183: f64, t70391: f64, t70497: f64, t75836: f64, t76576: f64, t76616: f64, t76722: f64, t977: f64, t5392: f64, t5398: f64, t20217: f64, t10408: f64, t13995: f64, t14164: f64, t14187: f64, t1539: f64, t21396: f64, t21403: f64, t21512: f64, t21520: f64, t21526: f64, t21551: f64, t3039: f64, t42483: f64, t43361: f64, t4588: f64, t49929: f64, t5677: f64, t5681: f64, t5867: f64, t62284: f64, t70535: f64, t70554: f64, t70573: f64, t70597: f64, t10413: f64, t10876: f64, t21118: f64, t21391: f64, t42309: f64, t42388: f64, t42624: f64, t4342: f64, t4583: f64, t5909: f64, t61950: f64, t62360: f64, t70640: f64, t70655: f64, t70660: f64, t70665: f64, t70703: f64, t75912: f64, t998: f64, t21130: f64, t3062: f64, t42397: f64, t50181: f64, t62445: f64, t62494: f64, t62559: f64, t62565: f64, t70711: f64, t70724: f64, t70766: f64, t70792: f64, t70800: f64, t70805: f64, t76589: f64, t10214: f64, t14172: f64, t21134: f64, t21566: f64, t21570: f64, t21574: f64, t21595: f64, t2979: f64, t43253: f64, t43307: f64, t50425: f64, t62832: f64, t70846: f64, t70867: f64, t70912: f64, t70929: f64, t76585: f64, t76593: f64, t76608: f64, t76624: f64, t76768: f64, t77498: f64, t1625: f64, t21390: f64, t5872: f64, t6739: f64, t1615: f64, t3188: f64, t5914: f64, t381: f64, t11046: f64, t11048: f64, t11065: f64, t1610: f64, t1632: f64, t21481: f64, t21615: f64, t21622: f64, t21627: f64, t21634: f64, t21647: f64, t3186: f64, t3200: f64, t3201: f64, t43553: f64, t43554: f64, t4669: f64, t47841: f64, t5936: f64, t1058: f64, t1060: f64, t14608: f64, t14618: f64, t1630: f64, t18086: f64, t21594: f64, t21614: f64, t21617: f64, t21644: f64, t21650: f64, t21653: f64, t43503: f64, t43505: f64, t47857: f64, t5937: f64, t69924: f64, t77485: f64, t1052: f64, t1055: f64, t11059: f64, t11060: f64, t11066: f64, t1603: f64, t1635: f64, t17575: f64, t17588: f64, t18074: f64, t21480: f64, t21618: f64, t21623: f64, t21635: f64, t21638: f64, t21657: f64, t21663: f64, t21677: f64, t21692: f64, t349: f64, t353: f64, t383: f64, t384: f64, t388: f64, t43515: f64, t43516: f64, t43576: f64, t43577: f64, t4557: f64, t4660: f64, t47853: f64, t5866: f64, t5903: f64, t5920: f64, t5928: f64, t5929: f64, t5933: f64, t5939: f64, t5941: f64, t5944: f64, t63004: f64, t63183: f64, t70987: f64, t76976: f64, t76977: f64, t1070: f64, t193: f64, t336: f64, t43637: f64, t76668: f64, t76671: f64, t76674: f64, t76675: f64, t76715: f64, t76997: f64, t77001: f64, t77003: f64, t77006: f64, t77009: f64, t77012: f64, t77014: f64, t77016: f64) -> (f64, f64, f64) {
        let t77539 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1386(t1021, t10403, t1041, t10480, t10883, t10970, t1409, t17712, t21405, t21532, t248, t3071, t3146, t360, t42358, t4582, t48670, t48674, t49934, t50193, t5878, t61782, t62079, t62840, t70100, t70239, t70346, t70351, t70363, t70389, t70404, t75847, t76581, t76740, t973, t974);
        let t77587 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1387(t1021, t10403, t1041, t1044, t14211, t14508, t14511, t1616, t21138, t21487, t21503, t21597, t21603, t21609, t248, t3070, t3071, t3130, t3131, t3151, t42444, t4582, t4641, t4644, t5685, t5873, t62137, t62148, t62177, t62183, t70391, t70497, t75836, t75847, t76576, t76616, t76722, t973, t974, t977);
        let t77606 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1388(t5392, t5398);
        let (t77621, t77637) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1389(t1409, t20217, t10403, t10408, t1041, t13995, t14164, t14187, t1539, t1616, t21396, t21403, t21512, t21520, t21526, t21551, t3039, t3070, t3071, t42483, t43361, t4582, t4588, t4644, t49929, t5677, t5681, t5867, t5873, t62284, t70391, t70535, t70554, t70573, t70597, t77606);
        let t77687 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1390(t10403, t10408, t1041, t10413, t10876, t1539, t1616, t17712, t21118, t21391, t3070, t3071, t42309, t42388, t42624, t4342, t4582, t4583, t5398, t5681, t5685, t5873, t5878, t5909, t61950, t62360, t70640, t70655, t70660, t70665, t70703, t75836, t75912, t77621, t973, t974, t998);
        let t77724 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1391(t10408, t1041, t10413, t1616, t21130, t248, t3062, t3070, t3071, t42397, t50181, t5677, t5681, t5685, t5867, t5878, t62445, t62494, t62559, t62565, t70711, t70724, t70766, t70792, t70800, t70805, t76589);
        let t77761 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1392(t10214, t1041, t13995, t14172, t1539, t1616, t21134, t21566, t21570, t21574, t21595, t2979, t3070, t3071, t43253, t43307, t4582, t50425, t62832, t70846, t70867, t70912, t70929, t76585, t76593, t76608, t76624, t77606, t973, t977);
        let (t77764, t77782, t77794) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1393(t76768, t77498, t77539, t77587, t77637, t77687, t77724, t77761, t1625, t21390, t5872, t6739);
        let (t77806, t77826, t77835) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1394(t1615, t3188, t5872, t5914, t381, t76740, t11046, t11048, t11065, t1610, t1632, t21481, t21615, t21622, t21627, t21634, t21647, t3131, t3186, t3200, t3201, t43553, t43554, t4669, t47841, t5936, t77782, t77794);
        let (t77855, t77892) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1395(t381, t76722, t1058, t1060, t14608, t14618, t1615, t1625, t1630, t18086, t21594, t21614, t21617, t21644, t21650, t21653, t3186, t3188, t43503, t43505, t47857, t5937, t69924, t77485, t77806, t77826);
        let t77913 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1396(t1052, t1055, t1058, t1060, t11046, t11059, t11060, t11065, t11066, t14608, t14618, t1603, t1625, t1635, t17575, t17588, t18074, t18086, t21480, t21614, t21617, t21618, t21622, t21623, t21635, t21638, t21657, t21663, t21677, t21692, t3200, t3201, t349, t353, t360, t381, t383, t384, t388, t43515, t43516, t43576, t43577, t4557, t4660, t4669, t47853, t5866, t5903, t5914, t5920, t5928, t5929, t5933, t5936, t5939, t5941, t5944, t63004, t63183, t70987, t76976, t76977, t77764, t77782, t77794, t77826, t77835, t77855, t77892);
        let t77918 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1397(t1070, t193, t336, t43637, t76668, t76671, t76674, t76675, t76715, t76997, t77001, t77003, t77006, t77009, t77012, t77014, t77016, t77913);
    (t77606, t77621, t77918)
}
