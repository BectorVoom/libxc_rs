//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta627 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2266;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2267;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2268;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2269;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2270;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2271;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2272;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2273;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2274;
use chunk9::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2275;
use chunk10::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2276;
use chunk11::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2277;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta627(t41008: f64, t4155: f64, t13076: f64, t9638: f64, t13322: f64, t13316: f64, t41115: f64, t4240: f64, t13278: f64, t2686: f64, t13173: f64, t2639: f64, t13186: f64, t13242: f64, t16836: f64, t2623: f64, t2643: f64, t41084: f64, t41086: f64, t41088: f64, t4167: f64, t9634: f64, t9646: f64, t9647: f64, t9663: f64, t1512: f64, t41340: f64, t4236: f64, t9671: f64, t4166: f64, t9973: f64, t41354: f64, t13198: f64, t2697: f64, t13302: f64, t13306: f64, t12971: f64, t13283: f64, t13300: f64, t1484: f64, t2553: f64, t2645: f64, t2684: f64, t2701: f64, t4119: f64, t41399: f64, t776: f64, t820: f64, t843: f64, t9516: f64, t9613: f64, t9978: f64, t9983: f64, t13248: f64, t13258: f64, t2631: f64, t4233: f64, t828: f64, t10007: f64, t13222: f64, t13223: f64, t13326: f64, t13350: f64, t1510: f64, t232: f64, t2647: f64, t41063: f64, t41096: f64, t41108: f64, t4178: f64, t4181: f64, t4182: f64, t46692: f64, t46693: f64, t9616: f64, t9642: f64, t13084: f64, t13353: f64, t41466: f64, t13176: f64, t2642: f64, t10024: f64, t1500: f64, t13293: f64, t9573: f64, t13005: f64, t13184: f64, t13196: f64, t13203: f64, t210: f64, t221: f64, t2571: f64, t2649: f64, t41014: f64, t41116: f64, t4180: f64, t4248: f64, t46644: f64, t46839: f64, t829: f64, t9632: f64, t9981: f64, t2379: f64, t4191: f64, t41107: f64, t9670: f64, t831: f64, t13210: f64, t13228: f64, t13254: f64, t13333: f64, t41130: f64, t41132: f64, t41134: f64, t41139: f64, t41237: f64, t41341: f64, t4172: f64, t9618: f64, t9960: f64, t39249: f64, t39256: f64, t39309: f64, t39312: f64, t39316: f64, t39320: f64, t40679: f64, t46120: f64, t46126: f64, t46129: f64, t46131: f64, t46133: f64, t46135: f64, t46138: f64, t46140: f64, t46141: f64, t46142: f64, t39373: f64, t39397: f64, t39400: f64, t39408: f64, t39411: f64, t40685: f64, t40689: f64, t40708: f64, t40714: f64, t40716: f64, t46143: f64, t46144: f64, t46152: f64, t46194: f64, t46195: f64, t46197: f64, t46207: f64, t39463: f64, t39468: f64, t39472: f64, t39476: f64, t39483: f64, t40721: f64, t40732: f64, t46209: f64, t46218: f64, t46228: f64, t46232: f64, t46235: f64, t46237: f64, t46238: f64, t46239: f64, t46245: f64, t46256: f64, t39529: f64, t40741: f64, t40743: f64, t40748: f64, t40760: f64, t40764: f64, t40766: f64, t46269: f64, t46279: f64, t46280: f64, t46282: f64, t46284: f64, t46286: f64, t46287: f64, t46288: f64, t46292: f64, t46293: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46912, t46918, t46920, t46926, t46929, t46930, t46936) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2266(t41008, t4155, t13076, t9638, t13322, t13316, t41115, t4240, t13278, t2686, t13173, t2639);
        let t46938 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2267(t13186, t13242, t16836, t2623, t2643, t41084, t41086, t41088, t4167, t46912, t46918, t46920, t46926, t46929, t46930, t46936, t9634, t9646, t9647, t9663);
        let (t46952, t46954, t46957, t46960, t46962, t46974, t46980) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2268(t1512, t41340, t4236, t9671, t4166, t9973, t41354, t13198, t2697, t13302, t9638, t13306);
        let t46982 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2269(t12971, t13283, t13300, t1484, t1512, t2553, t2643, t2645, t2684, t2701, t4119, t41399, t4236, t46952, t46954, t46957, t46960, t46962, t46974, t46980, t776, t820, t843, t9516, t9613, t9978, t9983);
        let (t47012, t47025) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2270(t13248, t13258, t1484, t2631, t4233, t828, t10007, t13076, t13222, t13223, t13242, t13322, t13326, t13350, t1510, t232, t2643, t2645, t2647, t41063, t41096, t41108, t4178, t4181, t4182, t4240, t46692, t46693, t9516, t9616, t9642);
        let (t47027, t47037, t47039, t47044, t47047, t47049) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2271(t13084, t13258, t13353, t9638, t41466, t820, t13176, t2642, t10024, t1500, t13293, t9573);
        let t47071 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2272(t13005, t13184, t13196, t13203, t13222, t13242, t13350, t210, t221, t2571, t2643, t2645, t2649, t41014, t41116, t4178, t4180, t4181, t4182, t4248, t46644, t46839, t47027, t47037, t47039, t47044, t47047, t47049, t776, t829, t9632, t9981);
        let t47097 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2273(t2379, t828, t41115, t4191, t41107, t4166, t9670, t831, t13210, t13228, t13254, t13333, t13350, t41130, t41132, t41134, t41139, t41237, t41341, t4167, t4172, t4178, t9618, t9642, t9960);
        let t47138 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2274(t39249, t39256, t39309, t39312, t39316, t39320, t40679, t46120, t46126, t46129, t46131, t46133, t46135, t46138, t46140, t46141, t46142);
        let t47139 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2275(t39373, t39397, t39400, t39408, t39411, t40685, t40689, t40708, t40714, t40716, t46143, t46144, t46152, t46194, t46195, t46197, t46207);
        let t47141 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2276(t39463, t39468, t39472, t39476, t39483, t40721, t40732, t46209, t46218, t46228, t46232, t46235, t46237, t46238, t46239, t46245, t46256);
        let t47142 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2277(t39529, t40741, t40743, t40748, t40760, t40764, t40766, t46269, t46279, t46280, t46282, t46284, t46286, t46287, t46288, t46292, t46293);
    (t46938, t46982, t47012, t47025, t47071, t47097, t47138, t47139, t47141, t47142)
}
