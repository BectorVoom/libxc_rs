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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta704(t23384: f64, t28684: f64, t1052: f64, t1065: f64, t17843: f64, t18074: f64, t1922: f64, t23346: f64, t25453: f64, t25743: f64, t28491: f64, t28500: f64, t28678: f64, t28681: f64, t3174: f64, t349: f64, t388: f64, t4552: f64, t4557: f64, t4660: f64, t6687: f64, t6776: f64, t7593: f64, t82469: f64, t83368: f64, t88937: f64, t88954: f64, t99859: f64, t1920: f64, t28474: f64, t968: f64, t14529: f64, t14555: f64, t1599: f64, t17187: f64, t1956: f64, t23372: f64, t25766: f64, t28485: f64, t3026: f64, t4542: f64, t5920: f64, t61061: f64, t6689: f64, t6690: f64, t7561: f64, t7600: f64, t7625: f64, t88182: f64, t89561: f64, t89583: f64, t89597: f64, t5914: f64, t6703: f64, t5843: f64, t984: f64, t1635: f64, t18062: f64, t18165: f64, t1955: f64, t23365: f64, t23588: f64, t25406: f64, t25732: f64, t25738: f64, t25778: f64, t25797: f64, t28480: f64, t4694: f64, t5844: f64, t63215: f64, t6706: f64, t6771: f64, t89609: f64, t89617: f64, t89666: f64, t986: f64, t28492: f64, t1625: f64, t18071: f64, t23327: f64, t25429: f64, t25431: f64, t25712: f64, t28691: f64, t343: f64, t7553: f64, t83444: f64, t88050: f64, t88105: f64, t89630: f64, t89648: f64, t89653: f64, t28648: f64, t82431: f64, t28667: f64, t82736: f64, t23665: f64, t28626: f64, t18080: f64, t18161: f64, t23601: f64, t23670: f64, t23677: f64, t23678: f64, t25470: f64, t25717: f64, t6797: f64, t6799: f64, t6800: f64, t82402: f64, t82534: f64, t88992: f64, t88998: f64, t28651: f64, t607: f64, t1539: f64, t7582: f64, t82655: f64, t28622: f64, t17635: f64, t18099: f64, t18154: f64, t23613: f64, t23633: f64, t25510: f64, t25511: f64, t25654: f64, t25721: f64, t28613: f64, t28671: f64, t82653: f64, t83233: f64, t83239: f64, t83240: f64, t83245: f64, t83246: f64, t89033: f64, t89399: f64, t3032: f64, t5872: f64, t1023: f64, t17686: f64, t17691: f64, t18150: f64, t23603: f64, t23604: f64, t25475: f64, t25485: f64, t25491: f64, t28617: f64, t28670: f64, t4594: f64, t4650: f64, t7603: f64, t82513: f64, t82683: f64, t89076: f64, t89210: f64, t89468: f64, t1011: f64, t5866: f64, t1948: f64, t7577: f64, t23657: f64, t25484: f64, t25502: f64, t25523: f64, t25540: f64, t25544: f64, t25660: f64, t25722: f64, t28621: f64, t7610: f64, t83265: f64, t89002: f64, t89049: f64, t89057: f64, t89395: f64, t225: f64, t28557: f64, t28565: f64, t11059: f64, t11065: f64, t17671: f64, t17732: f64, t18088: f64, t18103: f64, t18111: f64, t25516: f64, t25517: f64, t28596: f64, t28666: f64, t381: f64, t4347: f64, t4540: f64, t6784: f64, t6786: f64, t82620: f64, t89204: f64, t6743: f64, t28663: f64, t23511: f64, t5928: f64, t11037: f64, t23602: f64, t25486: f64, t25512: f64, t28597: f64, t28625: f64, t28657: f64, t3127: f64, t6801: f64, t82633: f64, t82635: f64, t884: f64, t89094: f64, t89104: f64, t28638: f64, t28605: f64, t1610: f64, t17876: f64, t1953: f64, t23685: f64, t23696: f64, t25706: f64, t28641: f64, t3200: f64, t4615: f64, t4684: f64, t5677: f64, t7622: f64, t89151: f64, t89156: f64, t89158: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t99866 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2297(t23384, t28684, t1052, t1065, t17843, t18074, t1922, t23346, t25453, t25743, t28491, t28500, t28678, t28681, t3174, t349, t388, t4552, t4557, t4660, t6687, t6776, t7593, t82469, t83368, t88937, t88954, t99859);
        let t99894 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2298(t1920, t28474, t968, t14529, t14555, t1599, t17187, t1956, t23372, t25766, t28485, t3026, t4542, t5920, t61061, t6687, t6689, t6690, t7561, t7600, t7625, t88182, t89561, t89583, t89597);
        let (t99921, t99930) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2299(t5914, t6703, t5843, t984, t1052, t1635, t18062, t18165, t1955, t1956, t23365, t23588, t25406, t25732, t25738, t25778, t25797, t28474, t28480, t3174, t4660, t4694, t5844, t63215, t6687, t6706, t6771, t89609, t89617, t89666, t986);
        let t99959 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2300(t23384, t28492, t28500, t1599, t1625, t18071, t23327, t23346, t25429, t25431, t25712, t28684, t28691, t343, t6687, t6690, t6771, t7553, t83444, t88050, t88105, t89630, t89648, t89653);
        let t99983 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2301(t28648, t82431, t28667, t82736, t23665, t28626, t18080, t18161, t23327, t23601, t23670, t23677, t23678, t25470, t25717, t6797, t6799, t6800, t82402, t82534, t88992, t88998);
        let t100025 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2302(t28651, t607, t6800, t1539, t7582, t82655, t23665, t28622, t17635, t18099, t18154, t23327, t23613, t23633, t25429, t25510, t25511, t25654, t25721, t28613, t28671, t6797, t6799, t82534, t82653, t83233, t83239, t83240, t83245, t83246, t89033, t89399);
        let (t100027, t100068) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2303(t3032, t5872, t1023, t17686, t17691, t18080, t18150, t23327, t23601, t23603, t23604, t23613, t25470, t25475, t25485, t25491, t25510, t25511, t25721, t28617, t28670, t4594, t4650, t6797, t6799, t6800, t7603, t82513, t82683, t89076, t89210, t89468);
        let (t100087, t100103) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2304(t1011, t5866, t1948, t7577, t1023, t23601, t23657, t25429, t25484, t25491, t25502, t25523, t25540, t25544, t25660, t25722, t28621, t28651, t4594, t6797, t7610, t83245, t83265, t89002, t89033, t89049, t89057, t89395);
        let t100147 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2305(t225, t28557, t28565, t100027, t11059, t11065, t1599, t17671, t17732, t18088, t18103, t18111, t1948, t23327, t23601, t25470, t25484, t25485, t25516, t25517, t28596, t28666, t381, t4347, t4540, t6687, t6784, t6786, t6797, t6799, t6800, t82513, t82620, t89204);
        let t100176 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2306(t28565, t6743, t23384, t28663, t23511, t5928, t100087, t11037, t1625, t23327, t23346, t23601, t23602, t23657, t23678, t25486, t25512, t28597, t28625, t28657, t3127, t6797, t6801, t82633, t82635, t83245, t884, t89094, t89104);
        let t100195 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2307(t23384, t28638, t23665, t28605, t1610, t17876, t1953, t23346, t23685, t23696, t25706, t28641, t3200, t4615, t4684, t5677, t6687, t7622, t89151, t89156, t89158);
    (t99866, t99894, t99921, t99930, t99959, t99983, t100025, t100068, t100103, t100147, t100176, t100195)
}
