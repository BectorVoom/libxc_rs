//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta704 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2297;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2298;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2299;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2300;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2301;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2302;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2303;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2304;
use chunk8::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2305;
use chunk9::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2306;
use chunk10::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2307;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta704<F: Float>(t23384: F, t28684: F, t1052: F, t1065: F, t17843: F, t18074: F, t1922: F, t23346: F, t25453: F, t25743: F, t28491: F, t28500: F, t28678: F, t28681: F, t3174: F, t349: F, t388: F, t4552: F, t4557: F, t4660: F, t6687: F, t6776: F, t7593: F, t82469: F, t83368: F, t88937: F, t88954: F, t99859: F, t1920: F, t28474: F, t968: F, t14529: F, t14555: F, t1599: F, t17187: F, t1956: F, t23372: F, t25766: F, t28485: F, t3026: F, t4542: F, t5920: F, t61061: F, t6689: F, t6690: F, t7561: F, t7600: F, t7625: F, t88182: F, t89561: F, t89583: F, t89597: F, t5914: F, t6703: F, t5843: F, t984: F, t1635: F, t18062: F, t18165: F, t1955: F, t23365: F, t23588: F, t25406: F, t25732: F, t25738: F, t25778: F, t25797: F, t28480: F, t4694: F, t5844: F, t63215: F, t6706: F, t6771: F, t89609: F, t89617: F, t89666: F, t986: F, t28492: F, t1625: F, t18071: F, t23327: F, t25429: F, t25431: F, t25712: F, t28691: F, t343: F, t7553: F, t83444: F, t88050: F, t88105: F, t89630: F, t89648: F, t89653: F, t28648: F, t82431: F, t28667: F, t82736: F, t23665: F, t28626: F, t18080: F, t18161: F, t23601: F, t23670: F, t23677: F, t23678: F, t25470: F, t25717: F, t6797: F, t6799: F, t6800: F, t82402: F, t82534: F, t88992: F, t88998: F, t28651: F, t607: F, t1539: F, t7582: F, t82655: F, t28622: F, t17635: F, t18099: F, t18154: F, t23613: F, t23633: F, t25510: F, t25511: F, t25654: F, t25721: F, t28613: F, t28671: F, t82653: F, t83233: F, t83239: F, t83240: F, t83245: F, t83246: F, t89033: F, t89399: F, t3032: F, t5872: F, t1023: F, t17686: F, t17691: F, t18150: F, t23603: F, t23604: F, t25475: F, t25485: F, t25491: F, t28617: F, t28670: F, t4594: F, t4650: F, t7603: F, t82513: F, t82683: F, t89076: F, t89210: F, t89468: F, t1011: F, t5866: F, t1948: F, t7577: F, t23657: F, t25484: F, t25502: F, t25523: F, t25540: F, t25544: F, t25660: F, t25722: F, t28621: F, t7610: F, t83265: F, t89002: F, t89049: F, t89057: F, t89395: F, t225: F, t28557: F, t28565: F, t11059: F, t11065: F, t17671: F, t17732: F, t18088: F, t18103: F, t18111: F, t25516: F, t25517: F, t28596: F, t28666: F, t381: F, t4347: F, t4540: F, t6784: F, t6786: F, t82620: F, t89204: F, t6743: F, t28663: F, t23511: F, t5928: F, t11037: F, t23602: F, t25486: F, t25512: F, t28597: F, t28625: F, t28657: F, t3127: F, t6801: F, t82633: F, t82635: F, t884: F, t89094: F, t89104: F, t28638: F, t28605: F, t1610: F, t17876: F, t1953: F, t23685: F, t23696: F, t25706: F, t28641: F, t3200: F, t4615: F, t4684: F, t5677: F, t7622: F, t89151: F, t89156: F, t89158: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t99866 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2297::<F>(t23384, t28684, t1052, t1065, t17843, t18074, t1922, t23346, t25453, t25743, t28491, t28500, t28678, t28681, t3174, t349, t388, t4552, t4557, t4660, t6687, t6776, t7593, t82469, t83368, t88937, t88954, t99859);
        let t99894 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2298::<F>(t1920, t28474, t968, t14529, t14555, t1599, t17187, t1956, t23372, t25766, t28485, t3026, t4542, t5920, t61061, t6687, t6689, t6690, t7561, t7600, t7625, t88182, t89561, t89583, t89597);
        let (t99921, t99930) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2299::<F>(t5914, t6703, t5843, t984, t1052, t1635, t18062, t18165, t1955, t1956, t23365, t23588, t25406, t25732, t25738, t25778, t25797, t28474, t28480, t3174, t4660, t4694, t5844, t63215, t6687, t6706, t6771, t89609, t89617, t89666, t986);
        let t99959 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2300::<F>(t23384, t28492, t28500, t1599, t1625, t18071, t23327, t23346, t25429, t25431, t25712, t28684, t28691, t343, t6687, t6690, t6771, t7553, t83444, t88050, t88105, t89630, t89648, t89653);
        let t99983 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2301::<F>(t28648, t82431, t28667, t82736, t23665, t28626, t18080, t18161, t23327, t23601, t23670, t23677, t23678, t25470, t25717, t6797, t6799, t6800, t82402, t82534, t88992, t88998);
        let t100025 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2302::<F>(t28651, t607, t6800, t1539, t7582, t82655, t23665, t28622, t17635, t18099, t18154, t23327, t23613, t23633, t25429, t25510, t25511, t25654, t25721, t28613, t28671, t6797, t6799, t82534, t82653, t83233, t83239, t83240, t83245, t83246, t89033, t89399);
        let (t100027, t100068) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2303::<F>(t3032, t5872, t1023, t17686, t17691, t18080, t18150, t23327, t23601, t23603, t23604, t23613, t25470, t25475, t25485, t25491, t25510, t25511, t25721, t28617, t28670, t4594, t4650, t6797, t6799, t6800, t7603, t82513, t82683, t89076, t89210, t89468);
        let (t100087, t100103) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2304::<F>(t1011, t5866, t1948, t7577, t1023, t23601, t23657, t25429, t25484, t25491, t25502, t25523, t25540, t25544, t25660, t25722, t28621, t28651, t4594, t6797, t7610, t83245, t83265, t89002, t89033, t89049, t89057, t89395);
        let t100147 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2305::<F>(t225, t28557, t28565, t100027, t11059, t11065, t1599, t17671, t17732, t18088, t18103, t18111, t1948, t23327, t23601, t25470, t25484, t25485, t25516, t25517, t28596, t28666, t381, t4347, t4540, t6687, t6784, t6786, t6797, t6799, t6800, t82513, t82620, t89204);
        let t100176 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2306::<F>(t28565, t6743, t23384, t28663, t23511, t5928, t100087, t11037, t1625, t23327, t23346, t23601, t23602, t23657, t23678, t25486, t25512, t28597, t28625, t28657, t3127, t6797, t6801, t82633, t82635, t83245, t884, t89094, t89104);
        let t100195 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2307::<F>(t23384, t28638, t23665, t28605, t1610, t17876, t1953, t23346, t23685, t23696, t25706, t28641, t3200, t4615, t4684, t5677, t6687, t7622, t89151, t89156, t89158);
    (t99866, t99894, t99921, t99930, t99959, t99983, t100025, t100068, t100103, t100147, t100176, t100195)
}
