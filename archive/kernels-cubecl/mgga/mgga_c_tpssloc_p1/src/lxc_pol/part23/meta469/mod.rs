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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta469<F: Float>(t1021: F, t10403: F, t1041: F, t10480: F, t10883: F, t10970: F, t1409: F, t17712: F, t21405: F, t21532: F, t248: F, t3071: F, t3146: F, t360: F, t42358: F, t4582: F, t48670: F, t48674: F, t49934: F, t50193: F, t5878: F, t61782: F, t62079: F, t62840: F, t70100: F, t70239: F, t70346: F, t70351: F, t70363: F, t70389: F, t70404: F, t75847: F, t76581: F, t76740: F, t973: F, t974: F, t1044: F, t14211: F, t14508: F, t14511: F, t1616: F, t21138: F, t21487: F, t21503: F, t21597: F, t21603: F, t21609: F, t3070: F, t3130: F, t3131: F, t3151: F, t42444: F, t4641: F, t4644: F, t5685: F, t5873: F, t62137: F, t62148: F, t62177: F, t62183: F, t70391: F, t70497: F, t75836: F, t76576: F, t76616: F, t76722: F, t977: F, t5392: F, t5398: F, t20217: F, t10408: F, t13995: F, t14164: F, t14187: F, t1539: F, t21396: F, t21403: F, t21512: F, t21520: F, t21526: F, t21551: F, t3039: F, t42483: F, t43361: F, t4588: F, t49929: F, t5677: F, t5681: F, t5867: F, t62284: F, t70535: F, t70554: F, t70573: F, t70597: F, t10413: F, t10876: F, t21118: F, t21391: F, t42309: F, t42388: F, t42624: F, t4342: F, t4583: F, t5909: F, t61950: F, t62360: F, t70640: F, t70655: F, t70660: F, t70665: F, t70703: F, t75912: F, t998: F, t21130: F, t3062: F, t42397: F, t50181: F, t62445: F, t62494: F, t62559: F, t62565: F, t70711: F, t70724: F, t70766: F, t70792: F, t70800: F, t70805: F, t76589: F, t10214: F, t14172: F, t21134: F, t21566: F, t21570: F, t21574: F, t21595: F, t2979: F, t43253: F, t43307: F, t50425: F, t62832: F, t70846: F, t70867: F, t70912: F, t70929: F, t76585: F, t76593: F, t76608: F, t76624: F, t76768: F, t77498: F, t1625: F, t21390: F, t5872: F, t6739: F, t1615: F, t3188: F, t5914: F, t381: F, t11046: F, t11048: F, t11065: F, t1610: F, t1632: F, t21481: F, t21615: F, t21622: F, t21627: F, t21634: F, t21647: F, t3186: F, t3200: F, t3201: F, t43553: F, t43554: F, t4669: F, t47841: F, t5936: F, t1058: F, t1060: F, t14608: F, t14618: F, t1630: F, t18086: F, t21594: F, t21614: F, t21617: F, t21644: F, t21650: F, t21653: F, t43503: F, t43505: F, t47857: F, t5937: F, t69924: F, t77485: F, t1052: F, t1055: F, t11059: F, t11060: F, t11066: F, t1603: F, t1635: F, t17575: F, t17588: F, t18074: F, t21480: F, t21618: F, t21623: F, t21635: F, t21638: F, t21657: F, t21663: F, t21677: F, t21692: F, t349: F, t353: F, t383: F, t384: F, t388: F, t43515: F, t43516: F, t43576: F, t43577: F, t4557: F, t4660: F, t47853: F, t5866: F, t5903: F, t5920: F, t5928: F, t5929: F, t5933: F, t5939: F, t5941: F, t5944: F, t63004: F, t63183: F, t70987: F, t76976: F, t76977: F, t1070: F, t193: F, t336: F, t43637: F, t76668: F, t76671: F, t76674: F, t76675: F, t76715: F, t76997: F, t77001: F, t77003: F, t77006: F, t77009: F, t77012: F, t77014: F, t77016: F) -> (F, F, F) {
        let t77539 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1386::<F>(t1021, t10403, t1041, t10480, t10883, t10970, t1409, t17712, t21405, t21532, t248, t3071, t3146, t360, t42358, t4582, t48670, t48674, t49934, t50193, t5878, t61782, t62079, t62840, t70100, t70239, t70346, t70351, t70363, t70389, t70404, t75847, t76581, t76740, t973, t974);
        let t77587 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1387::<F>(t1021, t10403, t1041, t1044, t14211, t14508, t14511, t1616, t21138, t21487, t21503, t21597, t21603, t21609, t248, t3070, t3071, t3130, t3131, t3151, t42444, t4582, t4641, t4644, t5685, t5873, t62137, t62148, t62177, t62183, t70391, t70497, t75836, t75847, t76576, t76616, t76722, t973, t974, t977);
        let t77606 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1388::<F>(t5392, t5398);
        let (t77621, t77637) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1389::<F>(t1409, t20217, t10403, t10408, t1041, t13995, t14164, t14187, t1539, t1616, t21396, t21403, t21512, t21520, t21526, t21551, t3039, t3070, t3071, t42483, t43361, t4582, t4588, t4644, t49929, t5677, t5681, t5867, t5873, t62284, t70391, t70535, t70554, t70573, t70597, t77606);
        let t77687 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1390::<F>(t10403, t10408, t1041, t10413, t10876, t1539, t1616, t17712, t21118, t21391, t3070, t3071, t42309, t42388, t42624, t4342, t4582, t4583, t5398, t5681, t5685, t5873, t5878, t5909, t61950, t62360, t70640, t70655, t70660, t70665, t70703, t75836, t75912, t77621, t973, t974, t998);
        let t77724 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1391::<F>(t10408, t1041, t10413, t1616, t21130, t248, t3062, t3070, t3071, t42397, t50181, t5677, t5681, t5685, t5867, t5878, t62445, t62494, t62559, t62565, t70711, t70724, t70766, t70792, t70800, t70805, t76589);
        let t77761 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1392::<F>(t10214, t1041, t13995, t14172, t1539, t1616, t21134, t21566, t21570, t21574, t21595, t2979, t3070, t3071, t43253, t43307, t4582, t50425, t62832, t70846, t70867, t70912, t70929, t76585, t76593, t76608, t76624, t77606, t973, t977);
        let (t77764, t77782, t77794) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1393::<F>(t76768, t77498, t77539, t77587, t77637, t77687, t77724, t77761, t1625, t21390, t5872, t6739);
        let (t77806, t77826, t77835) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1394::<F>(t1615, t3188, t5872, t5914, t381, t76740, t11046, t11048, t11065, t1610, t1632, t21481, t21615, t21622, t21627, t21634, t21647, t3131, t3186, t3200, t3201, t43553, t43554, t4669, t47841, t5936, t77782, t77794);
        let (t77855, t77892) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1395::<F>(t381, t76722, t1058, t1060, t14608, t14618, t1615, t1625, t1630, t18086, t21594, t21614, t21617, t21644, t21650, t21653, t3186, t3188, t43503, t43505, t47857, t5937, t69924, t77485, t77806, t77826);
        let t77913 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1396::<F>(t1052, t1055, t1058, t1060, t11046, t11059, t11060, t11065, t11066, t14608, t14618, t1603, t1625, t1635, t17575, t17588, t18074, t18086, t21480, t21614, t21617, t21618, t21622, t21623, t21635, t21638, t21657, t21663, t21677, t21692, t3200, t3201, t349, t353, t360, t381, t383, t384, t388, t43515, t43516, t43576, t43577, t4557, t4660, t4669, t47853, t5866, t5903, t5914, t5920, t5928, t5929, t5933, t5936, t5939, t5941, t5944, t63004, t63183, t70987, t76976, t76977, t77764, t77782, t77794, t77826, t77835, t77855, t77892);
        let t77918 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1397::<F>(t1070, t193, t336, t43637, t76668, t76671, t76674, t76675, t76715, t76997, t77001, t77003, t77006, t77009, t77012, t77014, t77016, t77913);
    (t77606, t77621, t77918)
}
