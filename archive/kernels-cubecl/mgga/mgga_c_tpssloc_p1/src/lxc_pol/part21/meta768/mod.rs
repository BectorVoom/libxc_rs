//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta768 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2653;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2654;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2655;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2656;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2657;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2658;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2659;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2660;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2661;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2662;
use chunk10::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2663;
use chunk11::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2664;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta768<F: Float>(t25: F, t28: F, t45870: F, zeta_threshold: F, t12633: F, t12636: F, t12648: F, t1426: F, t1434: F, t19331: F, t19334: F, t19335: F, t19338: F, t2252: F, t2255: F, t2283: F, t2304: F, t31: F, t3976: F, t4018: F, t5399: F, t5400: F, t5428: F, t5442: F, t628: F, t642: F, t65: F, t80: F, t3961: F, t3966: F, t12606: F, t12705: F, t1420: F, t19378: F, t19381: F, t2262: F, t2267: F, t2274: F, t2275: F, t2278: F, t39: F, t39210: F, t3981: F, t43: F, t45970: F, t45974: F, t51: F, t5408: F, t5411: F, t5416: F, t55: F, t615: F, t12695: F, t12699: F, t12702: F, t16558: F, t19368: F, t19369: F, t19372: F, t19377: F, t19390: F, t19397: F, t2244: F, t2250: F, t39159: F, t39168: F, t3990: F, t5392: F, t5398: F, t607: F, t9287: F, t9300: F, t12595: F, t12609: F, t12652: F, t19420: F, t19425: F, t19430: F, t19435: F, t2291: F, t2298: F, t39096: F, t39114: F, t4007: F, t4012: F, t634: F, t638: F, t9321: F, t9330: F, t12620: F, t12630: F, t12709: F, t1427: F, t19326: F, t19405: F, t19441: F, t2245: F, t2284: F, t33: F, t3998: F, t5393: F, t5427: F, t629: F, t66: F, t72: F, t19297: F, t604: F, t4021: F, t12571: F, t12585: F, t12588: F, t19299: F, t19310: F, t19318: F, t19445: F, t2235: F, t2240: F, t2241: F, t2307: F, t39054: F, t39063: F, t3958: F, t46104: F, t5389: F, t5445: F, t55631: F, t55673: F, t605: F, t645: F, t9228: F, t9231: F, t9239: F, t2239: F, t5385: F, t12568: F, t12582: F, t12719: F, t1437: F, t16: F, t19313: F, t39033: F, t39037: F, t39043: F, t39049: F, t3953: F, t45844: F, t46099: F, t86: F, t5: F, t112: F, t4025: F, t671: F, t111: F, t19449: F, t2319: F, t5449: F, t1441: F, t2363: F, t2311: F, t5456: F, t1268: F, t12725: F, t12734: F, t12739: F, t12813: F, t1458: F, t19451: F, t19456: F, t19534: F, t2314: F, t26114: F, t4028: F, t4072: F, t45632: F, t5113: F, t5493: F, t55410: F, t55568: F, t7676: F, t88: F, t9348: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t55677 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2653::<F>(t25, t28, t45870, zeta_threshold);
        let t55709 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2654::<F>(t12633, t12636, t12648, t1426, t1434, t19331, t19334, t19335, t19338, t2252, t2255, t2283, t2304, t31, t3976, t4018, t5399, t5400, t5428, t5442, t55677, t628, t642, t65, t80);
        let t55716 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2655::<F>(t3961, t3966);
        let t55723 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2656::<F>(t3966);
        let t55751 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2657::<F>(t12606, t12705, t1420, t19378, t19381, t2262, t2267, t2274, t2275, t2278, t39, t39210, t3981, t43, t45970, t45974, t51, t5408, t5411, t5416, t55, t55677, t55716, t55723, t615);
        let t55801 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2658::<F>(t12606, t12695, t12699, t12702, t1420, t16558, t19368, t19369, t19372, t19377, t19390, t19397, t2244, t2250, t2267, t2274, t39, t39159, t39168, t3990, t51, t5392, t5398, t607, t615, t9287, t9300);
        let t55867 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2659::<F>(t12595, t12606, t12609, t12652, t16558, t19420, t19425, t19430, t19435, t2244, t2250, t2291, t2298, t39096, t39114, t4007, t4012, t5392, t5398, t55677, t55723, t607, t634, t638, t9321, t9330);
        let t55875 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2660::<F>(t12620, t12630, t12709, t1427, t1434, t19326, t19405, t19441, t2244, t2245, t2283, t2284, t2304, t33, t3998, t4018, t5392, t5393, t5427, t5442, t55723, t55751, t55801, t55867, t629, t642, t65, t66, t72, t80);
        let t55888 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2661::<F>(t19297, t604, t4021, t12571, t12585, t12588, t19299, t19310, t19318, t19445, t2235, t2240, t2241, t2307, t39054, t39063, t3958, t46104, t5389, t5445, t55631, t55673, t55709, t55875, t605, t645, t9228, t9231, t9239);
        let t55924 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2662::<F>(t2239, t5385, t12568, t12582, t12719, t1437, t16, t19313, t19445, t2240, t2241, t2307, t39033, t39037, t39043, t39049, t3953, t3958, t4021, t45844, t46099, t5389, t5445, t645, t86, t9231, t9239);
        let (t55927, t55934, t55943, t55946, t55962, t55967) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2663::<F>(t5, t55888, t55924, t112, t4025, t671, t111, t19449, t2319, t5449, t1441, t2363, t2311, t5456);
        let t55969 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2664::<F>(t1268, t12725, t12734, t12739, t12813, t1458, t19451, t19456, t19534, t2314, t2363, t26114, t4028, t4072, t45632, t5113, t5493, t55410, t55568, t55927, t55934, t55943, t55946, t55962, t55967, t671, t7676, t88, t9348);
    (t55677, t55716, t55723, t55927, t55934, t55943, t55946, t55962, t55967, t55969)
}
