//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta631 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2296;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2297;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2298;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2299;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2300;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2301;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2302;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2303;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2304;
use chunk9::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2305;
use chunk10::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2306;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta631<F: Float>(t1519: F, t2678: F, t10091: F, t13176: F, t13381: F, t13390: F, t13431: F, t13456: F, t255: F, t2617: F, t2738: F, t2740: F, t41014: F, t4162: F, t4166: F, t4281: F, t4282: F, t4291: F, t4295: F, t46861: F, t812: F, t9958: F, t9981: F, t10016: F, t10058: F, t10069: F, t10076: F, t10077: F, t10094: F, t10098: F, t10112: F, t10115: F, t13050: F, t13053: F, t13059: F, t13065: F, t13072: F, t13171: F, t13263: F, t13336: F, t13380: F, t13384: F, t13388: F, t13393: F, t13397: F, t13398: F, t13417: F, t13429: F, t13433: F, t13448: F, t13450: F, t13453: F, t13461: F, t1499: F, t1510: F, t1523: F, t1525: F, t16830: F, t17034: F, t226: F, t235: F, t25168: F, t259: F, t2597: F, t2613: F, t2633: F, t2679: F, t2684: F, t2713: F, t2718: F, t2720: F, t2729: F, t2732: F, t2742: F, t2743: F, t40904: F, t40951: F, t41520: F, t4182: F, t4234: F, t4268: F, t4273: F, t4280: F, t4283: F, t4290: F, t4292: F, t4296: F, t4300: F, t46488: F, t46508: F, t46511: F, t46519: F, t46524: F, t46528: F, t47215: F, t47363: F, t47399: F, t47419: F, t47439: F, t47448: F, t47452: F, t47507: F, t808: F, t828: F, t829: F, t855: F, t858: F, t860: F, t861: F, t863: F, t866: F, t9584: F, t9593: F, t9612: F, t9661: F, t9976: F, t13068: F, t225: F, t13030: F, t10046: F, t10049: F, t10104: F, t10111: F, t13463: F, t1492: F, t1527: F, t1528: F, t40852: F, t40875: F, t40890: F, t4147: F, t41554: F, t4301: F, t13062: F, t13378: F, t10103: F, t10110: F, t10116: F, t13377: F, t218: F, t252: F, t2591: F, t2710: F, t2719: F, t4142: F, t4265: F, t46860: F, t798: F, t9590: F, t12895: F, t12971: F, t193: F, t202: F, t2522: F, t2553: F, t262: F, t4314: F, t46481: F, t47149: F, t47151: F, t47153: F, t47156: F, t47159: F, t47161: F, t47162: F, t47164: F, t776: F, t870: F, t2379: F, t1484: F, t40622: F, t4320: F, t47166: F, t47168: F, t47171: F, t47174: F, t47175: F, t47178: F, t47181: F, t47183: F, t47186: F, t10126: F, t10134: F, t12854: F, t12899: F, t13196: F, t13471: F, t13487: F, t16596: F, t1877: F, t2523: F, t2752: F, t39249: F, t39256: F, t39373: F, t39397: F, t39463: F, t39468: F, t39472: F, t39476: F, t39529: F, t40689: F, t40721: F, t40779: F, t40784: F, t40790: F, t40793: F, t4119: F, t41254: F, t41258: F, t41262: F, t4255: F, t4307: F, t4310: F, t4315: F, t46120: F, t46126: F, t46129: F, t46131: F, t46133: F, t46135: F, t46138: F, t46145: F, t46152: F, t46194: F, t46195: F, t46197: F, t46219: F, t46228: F, t46232: F, t46257: F, t46281: F, t46294: F, t46298: F, t46303: F, t46309: F, t46311: F, t46324: F, t46340: F, t46373: F, t46377: F, t46384: F, t46385: F, t46386: F, t46389: F, t46450: F, t868: F, t9470: F, t9516: F, t9616: F, t2: F, t2756: F, t584: F, t873: F, t13501: F, t16: F, t265: F, t4331: F, t591: F, t1409: F, t41666: F, t9288: F, t123: F, t41664: F, t10277: F, t2244: F, t3966: F, t2768: F, t12606: F, t2770: F, t607: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t47528, t47558) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2296::<F>(t1519, t2678, t10091, t13176, t13381, t13390, t13431, t13456, t255, t2617, t2738, t2740, t41014, t4162, t4166, t4281, t4282, t4291, t4295, t46861, t812, t9958, t9981);
        let t47564 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2297::<F>(t10016, t10058, t10069, t10076, t10077, t10094, t10098, t10112, t10115, t13050, t13053, t13059, t13065, t13072, t13171, t13176, t13263, t13336, t13380, t13384, t13388, t13390, t13393, t13397, t13398, t13417, t13429, t13433, t13448, t13450, t13453, t13461, t1499, t1510, t1519, t1523, t1525, t16830, t17034, t226, t235, t25168, t259, t2597, t2613, t2617, t2633, t2679, t2684, t2713, t2718, t2720, t2729, t2732, t2742, t2743, t40904, t40951, t41520, t4166, t4182, t4234, t4268, t4273, t4280, t4281, t4282, t4283, t4290, t4291, t4292, t4295, t4296, t4300, t46488, t46508, t46511, t46519, t46524, t46528, t47215, t47363, t47399, t47419, t47439, t47448, t47452, t47507, t47528, t47558, t808, t812, t828, t829, t855, t858, t860, t861, t863, t866, t9584, t9593, t9612, t9661, t9976);
        let t47593 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2298::<F>(t13068, t225, t13030, t10046, t10049, t10104, t10111, t10112, t13053, t13065, t13463, t1492, t1527, t1528, t259, t2720, t2743, t40852, t40875, t40890, t4147, t41554, t4268, t4301, t855, t866);
        let t47631 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2299::<F>(t13062, t225, t13378, t10049, t10103, t10110, t10116, t13059, t13377, t1527, t218, t252, t259, t2591, t2710, t2713, t2718, t2719, t4142, t4265, t4268, t4273, t4300, t4301, t46860, t47363, t798, t855, t866, t9590, t9593);
        let t47644 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2300::<F>(t12895, t12971, t193, t202, t2522, t2553, t262, t4314, t46481, t47149, t47151, t47153, t47156, t47159, t47161, t47162, t47164, t47564, t47593, t47631, t776, t870);
        let t47651 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2301::<F>(t193, t2379, t1484, t2522, t40622, t4320, t47166, t47168, t47171, t47174, t47175, t47178, t47181, t47183, t47186);
        let t47655 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2302::<F>(t10126, t10134, t12854, t12895, t12899, t13196, t13471, t13487, t16596, t1877, t2379, t2522, t2523, t2553, t2752, t39249, t39256, t39373, t39397, t39463, t39468, t39472, t39476, t39529, t40689, t40721, t40779, t40784, t40790, t40793, t4119, t41254, t41258, t41262, t4255, t4307, t4310, t4314, t4315, t46120, t46126, t46129, t46131, t46133, t46135, t46138, t46145, t46152, t46194, t46195, t46197, t46219, t46228, t46232, t46257, t46281, t46294, t46298, t46303, t46309, t46311, t46324, t46340, t46373, t46377, t46384, t46385, t46386, t46389, t46450, t47644, t47651, t868, t9470, t9516, t9616);
        let (t47668, t47670, t47672, t47674, t47676, t47679) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2303::<F>(t2, t2756, t584, t873, t13501, t16, t265, t4331, t591, t1409, t41666, t9288);
        let t47681 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2304::<F>(t123, t41664, t47679);
        let (t47684, t47686) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2305::<F>(t10277, t2244, t3966, t123, t2768);
        let (t47689, t47691) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2306::<F>(t12606, t2770, t607, t123, t2768);
    (t47655, t47668, t47670, t47672, t47674, t47676, t47679, t47681, t47684, t47686, t47689, t47691)
}
