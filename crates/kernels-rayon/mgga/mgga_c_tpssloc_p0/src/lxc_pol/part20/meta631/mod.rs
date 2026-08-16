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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta631(t1519: f64, t2678: f64, t10091: f64, t13176: f64, t13381: f64, t13390: f64, t13431: f64, t13456: f64, t255: f64, t2617: f64, t2738: f64, t2740: f64, t41014: f64, t4162: f64, t4166: f64, t4281: f64, t4282: f64, t4291: f64, t4295: f64, t46861: f64, t812: f64, t9958: f64, t9981: f64, t10016: f64, t10058: f64, t10069: f64, t10076: f64, t10077: f64, t10094: f64, t10098: f64, t10112: f64, t10115: f64, t13050: f64, t13053: f64, t13059: f64, t13065: f64, t13072: f64, t13171: f64, t13263: f64, t13336: f64, t13380: f64, t13384: f64, t13388: f64, t13393: f64, t13397: f64, t13398: f64, t13417: f64, t13429: f64, t13433: f64, t13448: f64, t13450: f64, t13453: f64, t13461: f64, t1499: f64, t1510: f64, t1523: f64, t1525: f64, t16830: f64, t17034: f64, t226: f64, t235: f64, t25168: f64, t259: f64, t2597: f64, t2613: f64, t2633: f64, t2679: f64, t2684: f64, t2713: f64, t2718: f64, t2720: f64, t2729: f64, t2732: f64, t2742: f64, t2743: f64, t40904: f64, t40951: f64, t41520: f64, t4182: f64, t4234: f64, t4268: f64, t4273: f64, t4280: f64, t4283: f64, t4290: f64, t4292: f64, t4296: f64, t4300: f64, t46488: f64, t46508: f64, t46511: f64, t46519: f64, t46524: f64, t46528: f64, t47215: f64, t47363: f64, t47399: f64, t47419: f64, t47439: f64, t47448: f64, t47452: f64, t47507: f64, t808: f64, t828: f64, t829: f64, t855: f64, t858: f64, t860: f64, t861: f64, t863: f64, t866: f64, t9584: f64, t9593: f64, t9612: f64, t9661: f64, t9976: f64, t13068: f64, t225: f64, t13030: f64, t10046: f64, t10049: f64, t10104: f64, t10111: f64, t13463: f64, t1492: f64, t1527: f64, t1528: f64, t40852: f64, t40875: f64, t40890: f64, t4147: f64, t41554: f64, t4301: f64, t13062: f64, t13378: f64, t10103: f64, t10110: f64, t10116: f64, t13377: f64, t218: f64, t252: f64, t2591: f64, t2710: f64, t2719: f64, t4142: f64, t4265: f64, t46860: f64, t798: f64, t9590: f64, t12895: f64, t12971: f64, t193: f64, t202: f64, t2522: f64, t2553: f64, t262: f64, t4314: f64, t46481: f64, t47149: f64, t47151: f64, t47153: f64, t47156: f64, t47159: f64, t47161: f64, t47162: f64, t47164: f64, t776: f64, t870: f64, t2379: f64, t1484: f64, t40622: f64, t4320: f64, t47166: f64, t47168: f64, t47171: f64, t47174: f64, t47175: f64, t47178: f64, t47181: f64, t47183: f64, t47186: f64, t10126: f64, t10134: f64, t12854: f64, t12899: f64, t13196: f64, t13471: f64, t13487: f64, t16596: f64, t1877: f64, t2523: f64, t2752: f64, t39249: f64, t39256: f64, t39373: f64, t39397: f64, t39463: f64, t39468: f64, t39472: f64, t39476: f64, t39529: f64, t40689: f64, t40721: f64, t40779: f64, t40784: f64, t40790: f64, t40793: f64, t4119: f64, t41254: f64, t41258: f64, t41262: f64, t4255: f64, t4307: f64, t4310: f64, t4315: f64, t46120: f64, t46126: f64, t46129: f64, t46131: f64, t46133: f64, t46135: f64, t46138: f64, t46145: f64, t46152: f64, t46194: f64, t46195: f64, t46197: f64, t46219: f64, t46228: f64, t46232: f64, t46257: f64, t46281: f64, t46294: f64, t46298: f64, t46303: f64, t46309: f64, t46311: f64, t46324: f64, t46340: f64, t46373: f64, t46377: f64, t46384: f64, t46385: f64, t46386: f64, t46389: f64, t46450: f64, t868: f64, t9470: f64, t9516: f64, t9616: f64, t2: f64, t2756: f64, t584: f64, t873: f64, t13501: f64, t16: f64, t265: f64, t4331: f64, t591: f64, t1409: f64, t41666: f64, t9288: f64, t123: f64, t41664: f64, t10277: f64, t2244: f64, t3966: f64, t2768: f64, t12606: f64, t2770: f64, t607: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47528, t47558) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2296(t1519, t2678, t10091, t13176, t13381, t13390, t13431, t13456, t255, t2617, t2738, t2740, t41014, t4162, t4166, t4281, t4282, t4291, t4295, t46861, t812, t9958, t9981);
        let t47564 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2297(t10016, t10058, t10069, t10076, t10077, t10094, t10098, t10112, t10115, t13050, t13053, t13059, t13065, t13072, t13171, t13176, t13263, t13336, t13380, t13384, t13388, t13390, t13393, t13397, t13398, t13417, t13429, t13433, t13448, t13450, t13453, t13461, t1499, t1510, t1519, t1523, t1525, t16830, t17034, t226, t235, t25168, t259, t2597, t2613, t2617, t2633, t2679, t2684, t2713, t2718, t2720, t2729, t2732, t2742, t2743, t40904, t40951, t41520, t4166, t4182, t4234, t4268, t4273, t4280, t4281, t4282, t4283, t4290, t4291, t4292, t4295, t4296, t4300, t46488, t46508, t46511, t46519, t46524, t46528, t47215, t47363, t47399, t47419, t47439, t47448, t47452, t47507, t47528, t47558, t808, t812, t828, t829, t855, t858, t860, t861, t863, t866, t9584, t9593, t9612, t9661, t9976);
        let t47593 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2298(t13068, t225, t13030, t10046, t10049, t10104, t10111, t10112, t13053, t13065, t13463, t1492, t1527, t1528, t259, t2720, t2743, t40852, t40875, t40890, t4147, t41554, t4268, t4301, t855, t866);
        let t47631 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2299(t13062, t225, t13378, t10049, t10103, t10110, t10116, t13059, t13377, t1527, t218, t252, t259, t2591, t2710, t2713, t2718, t2719, t4142, t4265, t4268, t4273, t4300, t4301, t46860, t47363, t798, t855, t866, t9590, t9593);
        let t47644 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2300(t12895, t12971, t193, t202, t2522, t2553, t262, t4314, t46481, t47149, t47151, t47153, t47156, t47159, t47161, t47162, t47164, t47564, t47593, t47631, t776, t870);
        let t47651 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2301(t193, t2379, t1484, t2522, t40622, t4320, t47166, t47168, t47171, t47174, t47175, t47178, t47181, t47183, t47186);
        let t47655 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2302(t10126, t10134, t12854, t12895, t12899, t13196, t13471, t13487, t16596, t1877, t2379, t2522, t2523, t2553, t2752, t39249, t39256, t39373, t39397, t39463, t39468, t39472, t39476, t39529, t40689, t40721, t40779, t40784, t40790, t40793, t4119, t41254, t41258, t41262, t4255, t4307, t4310, t4314, t4315, t46120, t46126, t46129, t46131, t46133, t46135, t46138, t46145, t46152, t46194, t46195, t46197, t46219, t46228, t46232, t46257, t46281, t46294, t46298, t46303, t46309, t46311, t46324, t46340, t46373, t46377, t46384, t46385, t46386, t46389, t46450, t47644, t47651, t868, t9470, t9516, t9616);
        let (t47668, t47670, t47672, t47674, t47676, t47679) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2303(t2, t2756, t584, t873, t13501, t16, t265, t4331, t591, t1409, t41666, t9288);
        let t47681 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2304(t123, t41664, t47679);
        let (t47684, t47686) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2305(t10277, t2244, t3966, t123, t2768);
        let (t47689, t47691) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2306(t12606, t2770, t607, t123, t2768);
    (t47655, t47668, t47670, t47672, t47674, t47676, t47679, t47681, t47684, t47686, t47689, t47691)
}
