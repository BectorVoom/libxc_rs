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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta627<F: Float>(t41008: F, t4155: F, t13076: F, t9638: F, t13322: F, t13316: F, t41115: F, t4240: F, t13278: F, t2686: F, t13173: F, t2639: F, t13186: F, t13242: F, t16836: F, t2623: F, t2643: F, t41084: F, t41086: F, t41088: F, t4167: F, t9634: F, t9646: F, t9647: F, t9663: F, t1512: F, t41340: F, t4236: F, t9671: F, t4166: F, t9973: F, t41354: F, t13198: F, t2697: F, t13302: F, t13306: F, t12971: F, t13283: F, t13300: F, t1484: F, t2553: F, t2645: F, t2684: F, t2701: F, t4119: F, t41399: F, t776: F, t820: F, t843: F, t9516: F, t9613: F, t9978: F, t9983: F, t13248: F, t13258: F, t2631: F, t4233: F, t828: F, t10007: F, t13222: F, t13223: F, t13326: F, t13350: F, t1510: F, t232: F, t2647: F, t41063: F, t41096: F, t41108: F, t4178: F, t4181: F, t4182: F, t46692: F, t46693: F, t9616: F, t9642: F, t13084: F, t13353: F, t41466: F, t13176: F, t2642: F, t10024: F, t1500: F, t13293: F, t9573: F, t13005: F, t13184: F, t13196: F, t13203: F, t210: F, t221: F, t2571: F, t2649: F, t41014: F, t41116: F, t4180: F, t4248: F, t46644: F, t46839: F, t829: F, t9632: F, t9981: F, t2379: F, t4191: F, t41107: F, t9670: F, t831: F, t13210: F, t13228: F, t13254: F, t13333: F, t41130: F, t41132: F, t41134: F, t41139: F, t41237: F, t41341: F, t4172: F, t9618: F, t9960: F, t39249: F, t39256: F, t39309: F, t39312: F, t39316: F, t39320: F, t40679: F, t46120: F, t46126: F, t46129: F, t46131: F, t46133: F, t46135: F, t46138: F, t46140: F, t46141: F, t46142: F, t39373: F, t39397: F, t39400: F, t39408: F, t39411: F, t40685: F, t40689: F, t40708: F, t40714: F, t40716: F, t46143: F, t46144: F, t46152: F, t46194: F, t46195: F, t46197: F, t46207: F, t39463: F, t39468: F, t39472: F, t39476: F, t39483: F, t40721: F, t40732: F, t46209: F, t46218: F, t46228: F, t46232: F, t46235: F, t46237: F, t46238: F, t46239: F, t46245: F, t46256: F, t39529: F, t40741: F, t40743: F, t40748: F, t40760: F, t40764: F, t40766: F, t46269: F, t46279: F, t46280: F, t46282: F, t46284: F, t46286: F, t46287: F, t46288: F, t46292: F, t46293: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t46912, t46918, t46920, t46926, t46929, t46930, t46936) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2266::<F>(t41008, t4155, t13076, t9638, t13322, t13316, t41115, t4240, t13278, t2686, t13173, t2639);
        let t46938 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2267::<F>(t13186, t13242, t16836, t2623, t2643, t41084, t41086, t41088, t4167, t46912, t46918, t46920, t46926, t46929, t46930, t46936, t9634, t9646, t9647, t9663);
        let (t46952, t46954, t46957, t46960, t46962, t46974, t46980) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2268::<F>(t1512, t41340, t4236, t9671, t4166, t9973, t41354, t13198, t2697, t13302, t9638, t13306);
        let t46982 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2269::<F>(t12971, t13283, t13300, t1484, t1512, t2553, t2643, t2645, t2684, t2701, t4119, t41399, t4236, t46952, t46954, t46957, t46960, t46962, t46974, t46980, t776, t820, t843, t9516, t9613, t9978, t9983);
        let (t47012, t47025) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2270::<F>(t13248, t13258, t1484, t2631, t4233, t828, t10007, t13076, t13222, t13223, t13242, t13322, t13326, t13350, t1510, t232, t2643, t2645, t2647, t41063, t41096, t41108, t4178, t4181, t4182, t4240, t46692, t46693, t9516, t9616, t9642);
        let (t47027, t47037, t47039, t47044, t47047, t47049) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2271::<F>(t13084, t13258, t13353, t9638, t41466, t820, t13176, t2642, t10024, t1500, t13293, t9573);
        let t47071 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2272::<F>(t13005, t13184, t13196, t13203, t13222, t13242, t13350, t210, t221, t2571, t2643, t2645, t2649, t41014, t41116, t4178, t4180, t4181, t4182, t4248, t46644, t46839, t47027, t47037, t47039, t47044, t47047, t47049, t776, t829, t9632, t9981);
        let t47097 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2273::<F>(t2379, t828, t41115, t4191, t41107, t4166, t9670, t831, t13210, t13228, t13254, t13333, t13350, t41130, t41132, t41134, t41139, t41237, t41341, t4167, t4172, t4178, t9618, t9642, t9960);
        let t47138 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2274::<F>(t39249, t39256, t39309, t39312, t39316, t39320, t40679, t46120, t46126, t46129, t46131, t46133, t46135, t46138, t46140, t46141, t46142);
        let t47139 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2275::<F>(t39373, t39397, t39400, t39408, t39411, t40685, t40689, t40708, t40714, t40716, t46143, t46144, t46152, t46194, t46195, t46197, t46207);
        let t47141 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2276::<F>(t39463, t39468, t39472, t39476, t39483, t40721, t40732, t46209, t46218, t46228, t46232, t46235, t46237, t46238, t46239, t46245, t46256);
        let t47142 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2277::<F>(t39529, t40741, t40743, t40748, t40760, t40764, t40766, t46269, t46279, t46280, t46282, t46284, t46286, t46287, t46288, t46292, t46293);
    (t46938, t46982, t47012, t47025, t47071, t47097, t47138, t47139, t47141, t47142)
}
