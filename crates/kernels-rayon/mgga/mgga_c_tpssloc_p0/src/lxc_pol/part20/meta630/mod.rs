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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta630(t47185: f64, t47149: f64, t47151: f64, t47153: f64, t47156: f64, t47159: f64, t47161: f64, t47162: f64, t47164: f64, t47166: f64, t47168: f64, t47171: f64, t47174: f64, t47175: f64, t47178: f64, t47181: f64, t47183: f64, t13151: f64, t13156: f64, t13157: f64, t1484: f64, t1504: f64, t1506: f64, t225: f64, t228: f64, t230: f64, t2667: f64, t2672: f64, t2675: f64, t4219: f64, t4225: f64, t4226: f64, t4230: f64, t46426: f64, t47138: f64, t47139: f64, t47141: f64, t47142: f64, t47145: f64, t47146: f64, t47148: f64, t6589: f64, t824: f64, t9458: f64, t9516: f64, t9616: f64, t9938: f64, t9954: f64, t12971: f64, t13141: f64, t13160: f64, t13161: f64, t13164: f64, t13167: f64, t16729: f64, t1891: f64, t232: f64, t2379: f64, t2553: f64, t4119: f64, t4227: f64, t68: f64, t776: f64, t822: f64, t825: f64, t845: f64, t9947: f64, t9951: f64, t46528: f64, t816: f64, t4159: f64, t9541: f64, t120: f64, t13173: f64, t13177: f64, t13193: f64, t13198: f64, t13302: f64, t2618: f64, t2623: f64, t2643: f64, t2645: f64, t2681: f64, t41355: f64, t41363: f64, t41365: f64, t41373: f64, t41386: f64, t817: f64, t819: f64, t820: f64, t829: f64, t831: f64, t9642: f64, t1509: f64, t2631: f64, t13360: f64, t2703: f64, t1516: f64, t41052: f64, t40961: f64, t4261: f64, t9993: f64, t4166: f64, t9600: f64, t849: f64, t13176: f64, t2696: f64, t13222: f64, t13228: f64, t13251: f64, t13300: f64, t13306: f64, t13350: f64, t2647: f64, t2679: f64, t41063: f64, t41090: f64, t4178: f64, t4248: f64, t4250: f64, t47012: f64, t9627: f64, t9653: f64, t9958: f64, t2707: f64, t9975: f64, t242: f64, t41347: f64, t812: f64, t40933: f64, t9660: f64, t10009: f64, t13262: f64, t13312: f64, t41078: f64, t41395: f64, t41397: f64, t41404: f64, t41415: f64, t41417: f64, t41425: f64, t41467: f64, t41468: f64, t4177: f64, t4180: f64, t4181: f64, t4184: f64, t46597: f64, t46692: f64, t9612: f64, t13297: f64, t9573: f64, t13080: f64, t9638: f64, t13365: f64, t210: f64, t41427: f64, t41435: f64, t41437: f64, t4158: f64, t4172: f64, t46693: f64, t843: f64, t847: f64, t9559: f64, t9976: f64, t9981: f64, t9997: f64, t46560: f64, t46593: f64, t46637: f64, t46670: f64, t46716: f64, t46868: f64, t46910: f64, t46938: f64, t46982: f64, t47025: f64, t47071: f64, t47097: f64, t2627: f64, t4265: f64, t226: f64, t40931: f64, t13377: f64, t814: f64, t10073: f64, t10081: f64, t13380: f64, t13397: f64, t13416: f64, t13423: f64, t2617: f64, t2633: f64, t2736: f64, t4281: f64, t4282: f64, t4288: f64, t13396: f64, t808: f64, t2710: f64, t4233: f64, t852: f64, t13170: f64, t252: f64, t10084: f64, t10101: f64, t13263: f64, t13384: f64, t13401: f64, t13404: f64, t13453: f64, t2684: f64, t2733: f64, t4182: f64, t4291: f64, t9661: f64, t10055: f64, t13385: f64, t13407: f64, t13414: f64, t13434: f64, t25236: f64, t2613: f64, t4286: f64, t4298: f64, t9632: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47186, t47187) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2284(t47185, t47149, t47151, t47153, t47156, t47159, t47161, t47162, t47164, t47166, t47168, t47171, t47174, t47175, t47178, t47181, t47183);
        let t47213 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2285(t13151, t13156, t13157, t1484, t1504, t1506, t225, t228, t230, t2667, t2672, t2675, t4219, t4225, t4226, t4230, t46426, t47138, t47139, t47141, t47142, t47145, t47146, t47148, t47187, t6589, t824, t9458, t9516, t9616, t9938, t9954);
        let t47215 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2286(t12971, t13141, t13151, t13160, t13161, t13164, t13167, t1504, t16729, t1891, t232, t2379, t2553, t2667, t4119, t4225, t4227, t47213, t68, t776, t822, t825, t845, t9947, t9951);
        let t47239 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2287(t46528, t816, t4159, t9541, t120, t12971, t13173, t13177, t13193, t13198, t13302, t2618, t2623, t2643, t2645, t2681, t41355, t41363, t41365, t41373, t41386, t47215, t817, t819, t820, t829, t831, t9642);
        let (t47262, t47267, t47270, t47271, t47273, t47276) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2288(t1509, t2631, t13360, t2703, t1516, t41052, t40961, t4261, t9993, t4166, t9600, t849);
        let t47281 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2289(t47276, t13176, t2696, t849, t13222, t13228, t13251, t13300, t13306, t13350, t2643, t2645, t2647, t2679, t41063, t41090, t4178, t4248, t4250, t47012, t47262, t47267, t47270, t47271, t47273, t9627, t9642, t9653, t9958);
        let (t47285, t47308, t47318) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2290(t13360, t2707, t1509, t9975, t242, t41347, t812, t40933, t9660, t10009, t13251, t13262, t13312, t2643, t2645, t2647, t41078, t41395, t41397, t41404, t41415, t41417, t41425, t41467, t41468, t4177, t4180, t4181, t4184, t46597, t46692, t9612, t9642);
        let t47359 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2291(t2631, t776, t13297, t9573, t13080, t9638, t13222, t13228, t13262, t13365, t210, t2379, t2643, t2647, t2707, t41427, t41435, t41437, t4158, t4172, t4178, t4180, t4181, t46426, t46693, t47285, t820, t843, t847, t9559, t9976, t9981, t9997);
        let t47363 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2292(t46560, t46593, t46637, t46670, t46716, t46868, t46910, t46938, t46982, t47025, t47071, t47097, t47239, t47281, t47318, t47359);
        let t47399 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2293(t2627, t4265, t226, t40931, t68, t13377, t814, t10073, t10081, t13176, t13380, t13397, t13416, t13423, t2617, t2633, t2736, t4166, t4281, t4282, t4288, t47308, t812, t829, t9612, t9976, t9981);
        let (t47419, t47425, t47439, t47448, t47452) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2294(t13396, t808, t1509, t2710, t4233, t852, t13170, t252, t10084, t10101, t13176, t13263, t13380, t13384, t13397, t13401, t13404, t13453, t2684, t2733, t4166, t4182, t4281, t4282, t4291, t829, t9661);
        let t47507 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2295(t10055, t13380, t13384, t13385, t13407, t13414, t13434, t13453, t25236, t2613, t2617, t2679, t4166, t4281, t4286, t4291, t4298, t47425, t829, t9612, t9632);
    (t47186, t47215, t47363, t47399, t47419, t47439, t47448, t47452, t47507)
}
