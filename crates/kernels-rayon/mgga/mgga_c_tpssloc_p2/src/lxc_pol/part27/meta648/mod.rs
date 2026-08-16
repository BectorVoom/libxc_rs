//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta648 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2238;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2239;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2240;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2241;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2242;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2243;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2244;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2245;
use chunk8::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2246;
use chunk9::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2247;
use chunk10::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2248;
use chunk11::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2249;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta648(t7611: f64, t82713: f64, t82716: f64, t3040: f64, t7593: f64, t25550: f64, t82822: f64, t23384: f64, t25476: f64, t1058: f64, t1060: f64, t13940: f64, t14488: f64, t14618: f64, t1945: f64, t1953: f64, t23701: f64, t25499: f64, t25516: f64, t25535: f64, t2776: f64, t3186: f64, t3200: f64, t3201: f64, t4615: f64, t4673: f64, t6687: f64, t6784: f64, t6797: f64, t6813: f64, t7610: f64, t82592: f64, t986: f64, t4541: f64, t984: f64, t25467: f64, t25459: f64, t11037: f64, t13933: f64, t14526: f64, t1615: f64, t1920: f64, t1948: f64, t1949: f64, t23346: f64, t23571: f64, t23670: f64, t25541: f64, t25558: f64, t25713: f64, t25718: f64, t3076: f64, t3188: f64, t345: f64, t7622: f64, t88941: f64, t7604: f64, t82632: f64, t25723: f64, t88810: f64, t1409: f64, t1539: f64, t6746: f64, t82655: f64, t14220: f64, t7581: f64, t11034: f64, t1599: f64, t1629: f64, t23518: f64, t23604: f64, t23620: f64, t23633: f64, t25567: f64, t25659: f64, t25708: f64, t82382: f64, t82653: f64, t82789: f64, t83233: f64, t83245: f64, t83265: f64, t89106: f64, t25555: f64, t25529: f64, t6680: f64, t2966: f64, t7614: f64, t14622: f64, t14651: f64, t1610: f64, t23478: f64, t23635: f64, t23685: f64, t23707: f64, t25712: f64, t4684: f64, t61774: f64, t6800: f64, t6811: f64, t7619: f64, t82566: f64, t82799: f64, t82806: f64, t25471: f64, t82431: f64, t7607: f64, t25490: f64, t82514: f64, t7577: f64, t1014: f64, t1023: f64, t1049: f64, t12648: f64, t12652: f64, t23327: f64, t23601: f64, t23602: f64, t23605: f64, t23705: f64, t23714: f64, t25429: f64, t25470: f64, t25485: f64, t25491: f64, t25492: f64, t25510: f64, t25554: f64, t25721: f64, t3041: f64, t3121: f64, t4669: f64, t4677: f64, t6743: f64, t82513: f64, t82809: f64, t89194: f64, t89205: f64, t83244: f64, t974: f64, t985: f64, t3030: f64, t343: f64, t25483: f64, t25486: f64, t1022: f64, t23678: f64, t25479: f64, t25705: f64, t4680: f64, t82668: f64, t82823: f64, t82828: f64, t82830: f64, t83246: f64, t88155: f64, t4547: f64, t82573: f64, t11051: f64, t11054: f64, t14608: f64, t23654: f64, t23662: f64, t25493: f64, t25518: f64, t25549: f64, t3016: f64, t353: f64, t383: f64, t4649: f64, t6768: f64, t6786: f64, t7620: f64, t82534: f64, t82625: f64, t88728: f64, t10165: f64, t1052: f64, t1055: f64, t13736: f64, t1634: f64, t1956: f64, t23378: f64, t23721: f64, t23722: f64, t25400: f64, t25743: f64, t25797: f64, t3026: f64, t3174: f64, t3175: f64, t4557: f64, t4660: f64, t50625: f64, t6771: f64, t7624: f64, t83358: f64, t83364: f64, t83368: f64, t83420: f64, t88954: f64, t89001: f64, t89042: f64, t89066: f64, t89101: f64, t89143: f64, t89181: f64, t89225: f64, t89265: f64, t89297: f64, t25766: f64, t968: f64, t25739: f64, t11010: f64, t14552: f64, t14555: f64, t1603: f64, t23329: f64, t25423: f64, t25430: f64, t25755: f64, t25767: f64, t3020: f64, t3169: f64, t3207: f64, t388: f64, t50632: f64, t6776: f64, t6816: f64, t7625: f64, t25751: f64, t4657: f64, t6703: f64, t7554: f64, t7561: f64, t225: f64, t25789: f64, t1066: f64, t13742: f64, t1635: f64, t23394: f64, t23588: f64, t25407: f64, t25732: f64, t4542: f64, t50653: f64, t50690: f64, t6704: f64, t6706: f64, t82402: f64, t83398: f64, t83408: f64, t25802: f64, t23587: f64, t7560: f64, t25410: f64, t14548: f64, t23341: f64, t25436: f64, t3206: f64, t83435: f64, t83441: f64, t83444: f64, t25798: f64, t25822: f64, t7557: f64, t10160: f64, t23353: f64, t23365: f64, t25403: f64, t25453: f64, t25738: f64, t7600: f64, t82442: f64, t82499: f64, t83457: f64, t83459: f64) -> (f64, f64, f64, f64, f64) {
        let (t89312, t89330) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2238(t7611, t82713, t82716, t3040, t7593, t25550, t82822, t23384, t25476, t1058, t1060, t13940, t14488, t14618, t1945, t1953, t23701, t25499, t25516, t25535, t2776, t3186, t3200, t3201, t4615, t4673, t6687, t6784, t6797, t6813, t7610, t82592, t986);
        let (t89349, t89363) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2239(t4541, t984, t23384, t25467, t25459, t1058, t1060, t11037, t13933, t14526, t1615, t1920, t1948, t1949, t23346, t23571, t23670, t25541, t25558, t25713, t25718, t3076, t3186, t3188, t345, t6687, t7622, t88941, t89312);
        let (t89375, t89402) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2240(t7604, t82632, t25723, t88810, t1409, t3040, t1539, t6746, t82655, t14220, t7581, t11034, t1599, t1629, t23346, t23518, t23604, t23620, t23633, t25467, t25567, t25659, t25708, t3186, t4673, t6687, t82382, t82653, t82789, t83233, t83245, t83265, t89106);
        let t89433 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2241(t25555, t82822, t25529, t6680, t1920, t2966, t7614, t14622, t14651, t1539, t1610, t23478, t23633, t23635, t23685, t23707, t25567, t25712, t3200, t4684, t61774, t6687, t6784, t6800, t6811, t7619, t82566, t82799, t82806);
        let t89477 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2242(t25471, t82431, t7607, t82632, t25490, t82514, t23518, t7577, t1014, t1023, t1049, t12648, t12652, t23327, t23601, t23602, t23605, t23633, t23705, t23714, t25429, t25470, t25485, t25491, t25492, t25510, t25554, t25721, t3041, t3121, t4669, t4677, t6743, t82513, t82809, t89194, t89205);
        let t89515 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2243(t83244, t974, t985, t3030, t343, t25483, t25486, t25490, t25492, t1022, t1058, t1060, t23633, t23670, t23678, t25479, t25499, t25554, t25555, t25705, t25713, t3200, t4680, t4684, t6687, t6743, t82668, t82823, t82828, t82830, t83245, t83246, t88155, t89375);
        let t89547 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2244(t23478, t4547, t7607, t82573, t1058, t1060, t11051, t11054, t14608, t23327, t23346, t23633, t23654, t23662, t25493, t25518, t25549, t3016, t3186, t353, t383, t4649, t4669, t6687, t6768, t6786, t7614, t7619, t7620, t82382, t82534, t82625, t88728);
        let t89556 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2245(t10165, t1052, t1055, t13736, t1599, t1634, t1956, t23346, t23378, t23721, t23722, t25400, t25743, t25797, t3026, t3174, t3175, t4557, t4660, t50625, t6687, t6771, t7624, t83358, t83364, t83368, t83420, t88941, t88954, t89001, t89042, t89066, t89101, t89143, t89181, t89225, t89265, t89297, t89330, t89363, t89402, t89433, t89477, t89515, t89547);
        let t89590 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2246(t1920, t25766, t968, t23384, t25739, t11010, t12652, t14552, t14555, t1603, t1956, t23327, t23329, t23571, t25423, t25429, t25430, t25743, t25755, t25767, t3020, t3169, t3207, t388, t50632, t6680, t6687, t6776, t6816, t7593, t7625, t986);
        let t89623 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2247(t25751, t82431, t4657, t6703, t7554, t82573, t1920, t2966, t7561, t225, t25789, t1066, t13742, t1635, t1956, t23346, t23394, t23588, t25407, t25732, t3169, t4542, t50653, t50690, t6687, t6704, t6706, t82402, t83398, t83408);
        let t89658 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2248(t23384, t25802, t23587, t7560, t25410, t1052, t14548, t23341, t23346, t23394, t25436, t25797, t3016, t3174, t3206, t4557, t6687, t6704, t7561, t7624, t83435, t83441, t83444, t89349, t986);
        let t89690 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2249(t23384, t25798, t225, t25822, t7557, t82632, t10160, t1066, t14555, t1599, t1635, t23346, t23353, t23365, t23378, t25403, t25453, t25738, t3169, t4557, t6687, t6816, t7600, t82442, t82499, t83457, t83459);
    (t89556, t89590, t89623, t89658, t89690)
}
