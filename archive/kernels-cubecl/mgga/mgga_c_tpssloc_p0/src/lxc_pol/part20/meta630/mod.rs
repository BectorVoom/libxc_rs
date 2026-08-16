//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta630 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2284;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2285;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2286;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2287;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2288;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2289;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2290;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2291;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2292;
use chunk9::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2293;
use chunk10::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2294;
use chunk11::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2295;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta630<F: Float>(t47185: F, t47149: F, t47151: F, t47153: F, t47156: F, t47159: F, t47161: F, t47162: F, t47164: F, t47166: F, t47168: F, t47171: F, t47174: F, t47175: F, t47178: F, t47181: F, t47183: F, t13151: F, t13156: F, t13157: F, t1484: F, t1504: F, t1506: F, t225: F, t228: F, t230: F, t2667: F, t2672: F, t2675: F, t4219: F, t4225: F, t4226: F, t4230: F, t46426: F, t47138: F, t47139: F, t47141: F, t47142: F, t47145: F, t47146: F, t47148: F, t6589: F, t824: F, t9458: F, t9516: F, t9616: F, t9938: F, t9954: F, t12971: F, t13141: F, t13160: F, t13161: F, t13164: F, t13167: F, t16729: F, t1891: F, t232: F, t2379: F, t2553: F, t4119: F, t4227: F, t68: F, t776: F, t822: F, t825: F, t845: F, t9947: F, t9951: F, t46528: F, t816: F, t4159: F, t9541: F, t120: F, t13173: F, t13177: F, t13193: F, t13198: F, t13302: F, t2618: F, t2623: F, t2643: F, t2645: F, t2681: F, t41355: F, t41363: F, t41365: F, t41373: F, t41386: F, t817: F, t819: F, t820: F, t829: F, t831: F, t9642: F, t1509: F, t2631: F, t13360: F, t2703: F, t1516: F, t41052: F, t40961: F, t4261: F, t9993: F, t4166: F, t9600: F, t849: F, t13176: F, t2696: F, t13222: F, t13228: F, t13251: F, t13300: F, t13306: F, t13350: F, t2647: F, t2679: F, t41063: F, t41090: F, t4178: F, t4248: F, t4250: F, t47012: F, t9627: F, t9653: F, t9958: F, t2707: F, t9975: F, t242: F, t41347: F, t812: F, t40933: F, t9660: F, t10009: F, t13262: F, t13312: F, t41078: F, t41395: F, t41397: F, t41404: F, t41415: F, t41417: F, t41425: F, t41467: F, t41468: F, t4177: F, t4180: F, t4181: F, t4184: F, t46597: F, t46692: F, t9612: F, t13297: F, t9573: F, t13080: F, t9638: F, t13365: F, t210: F, t41427: F, t41435: F, t41437: F, t4158: F, t4172: F, t46693: F, t843: F, t847: F, t9559: F, t9976: F, t9981: F, t9997: F, t46560: F, t46593: F, t46637: F, t46670: F, t46716: F, t46868: F, t46910: F, t46938: F, t46982: F, t47025: F, t47071: F, t47097: F, t2627: F, t4265: F, t226: F, t40931: F, t13377: F, t814: F, t10073: F, t10081: F, t13380: F, t13397: F, t13416: F, t13423: F, t2617: F, t2633: F, t2736: F, t4281: F, t4282: F, t4288: F, t13396: F, t808: F, t2710: F, t4233: F, t852: F, t13170: F, t252: F, t10084: F, t10101: F, t13263: F, t13384: F, t13401: F, t13404: F, t13453: F, t2684: F, t2733: F, t4182: F, t4291: F, t9661: F, t10055: F, t13385: F, t13407: F, t13414: F, t13434: F, t25236: F, t2613: F, t4286: F, t4298: F, t9632: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t47186, t47187) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2284::<F>(t47185, t47149, t47151, t47153, t47156, t47159, t47161, t47162, t47164, t47166, t47168, t47171, t47174, t47175, t47178, t47181, t47183);
        let t47213 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2285::<F>(t13151, t13156, t13157, t1484, t1504, t1506, t225, t228, t230, t2667, t2672, t2675, t4219, t4225, t4226, t4230, t46426, t47138, t47139, t47141, t47142, t47145, t47146, t47148, t47187, t6589, t824, t9458, t9516, t9616, t9938, t9954);
        let t47215 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2286::<F>(t12971, t13141, t13151, t13160, t13161, t13164, t13167, t1504, t16729, t1891, t232, t2379, t2553, t2667, t4119, t4225, t4227, t47213, t68, t776, t822, t825, t845, t9947, t9951);
        let t47239 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2287::<F>(t46528, t816, t4159, t9541, t120, t12971, t13173, t13177, t13193, t13198, t13302, t2618, t2623, t2643, t2645, t2681, t41355, t41363, t41365, t41373, t41386, t47215, t817, t819, t820, t829, t831, t9642);
        let (t47262, t47267, t47270, t47271, t47273, t47276) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2288::<F>(t1509, t2631, t13360, t2703, t1516, t41052, t40961, t4261, t9993, t4166, t9600, t849);
        let t47281 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2289::<F>(t47276, t13176, t2696, t849, t13222, t13228, t13251, t13300, t13306, t13350, t2643, t2645, t2647, t2679, t41063, t41090, t4178, t4248, t4250, t47012, t47262, t47267, t47270, t47271, t47273, t9627, t9642, t9653, t9958);
        let (t47285, t47308, t47318) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2290::<F>(t13360, t2707, t1509, t9975, t242, t41347, t812, t40933, t9660, t10009, t13251, t13262, t13312, t2643, t2645, t2647, t41078, t41395, t41397, t41404, t41415, t41417, t41425, t41467, t41468, t4177, t4180, t4181, t4184, t46597, t46692, t9612, t9642);
        let t47359 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2291::<F>(t2631, t776, t13297, t9573, t13080, t9638, t13222, t13228, t13262, t13365, t210, t2379, t2643, t2647, t2707, t41427, t41435, t41437, t4158, t4172, t4178, t4180, t4181, t46426, t46693, t47285, t820, t843, t847, t9559, t9976, t9981, t9997);
        let t47363 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2292::<F>(t46560, t46593, t46637, t46670, t46716, t46868, t46910, t46938, t46982, t47025, t47071, t47097, t47239, t47281, t47318, t47359);
        let t47399 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2293::<F>(t2627, t4265, t226, t40931, t68, t13377, t814, t10073, t10081, t13176, t13380, t13397, t13416, t13423, t2617, t2633, t2736, t4166, t4281, t4282, t4288, t47308, t812, t829, t9612, t9976, t9981);
        let (t47419, t47425, t47439, t47448, t47452) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2294::<F>(t13396, t808, t1509, t2710, t4233, t852, t13170, t252, t10084, t10101, t13176, t13263, t13380, t13384, t13397, t13401, t13404, t13453, t2684, t2733, t4166, t4182, t4281, t4282, t4291, t829, t9661);
        let t47507 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2295::<F>(t10055, t13380, t13384, t13385, t13407, t13414, t13434, t13453, t25236, t2613, t2617, t2679, t4166, t4281, t4286, t4291, t4298, t47425, t829, t9612, t9632);
    (t47186, t47215, t47363, t47399, t47419, t47439, t47448, t47452, t47507)
}
