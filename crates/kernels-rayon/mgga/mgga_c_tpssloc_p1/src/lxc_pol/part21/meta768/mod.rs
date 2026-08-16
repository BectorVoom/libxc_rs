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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta768(t25: f64, t28: f64, t45870: f64, zeta_threshold: f64, t12633: f64, t12636: f64, t12648: f64, t1426: f64, t1434: f64, t19331: f64, t19334: f64, t19335: f64, t19338: f64, t2252: f64, t2255: f64, t2283: f64, t2304: f64, t31: f64, t3976: f64, t4018: f64, t5399: f64, t5400: f64, t5428: f64, t5442: f64, t628: f64, t642: f64, t65: f64, t80: f64, t3961: f64, t3966: f64, t12606: f64, t12705: f64, t1420: f64, t19378: f64, t19381: f64, t2262: f64, t2267: f64, t2274: f64, t2275: f64, t2278: f64, t39: f64, t39210: f64, t3981: f64, t43: f64, t45970: f64, t45974: f64, t51: f64, t5408: f64, t5411: f64, t5416: f64, t55: f64, t615: f64, t12695: f64, t12699: f64, t12702: f64, t16558: f64, t19368: f64, t19369: f64, t19372: f64, t19377: f64, t19390: f64, t19397: f64, t2244: f64, t2250: f64, t39159: f64, t39168: f64, t3990: f64, t5392: f64, t5398: f64, t607: f64, t9287: f64, t9300: f64, t12595: f64, t12609: f64, t12652: f64, t19420: f64, t19425: f64, t19430: f64, t19435: f64, t2291: f64, t2298: f64, t39096: f64, t39114: f64, t4007: f64, t4012: f64, t634: f64, t638: f64, t9321: f64, t9330: f64, t12620: f64, t12630: f64, t12709: f64, t1427: f64, t19326: f64, t19405: f64, t19441: f64, t2245: f64, t2284: f64, t33: f64, t3998: f64, t5393: f64, t5427: f64, t629: f64, t66: f64, t72: f64, t19297: f64, t604: f64, t4021: f64, t12571: f64, t12585: f64, t12588: f64, t19299: f64, t19310: f64, t19318: f64, t19445: f64, t2235: f64, t2240: f64, t2241: f64, t2307: f64, t39054: f64, t39063: f64, t3958: f64, t46104: f64, t5389: f64, t5445: f64, t55631: f64, t55673: f64, t605: f64, t645: f64, t9228: f64, t9231: f64, t9239: f64, t2239: f64, t5385: f64, t12568: f64, t12582: f64, t12719: f64, t1437: f64, t16: f64, t19313: f64, t39033: f64, t39037: f64, t39043: f64, t39049: f64, t3953: f64, t45844: f64, t46099: f64, t86: f64, t5: f64, t112: f64, t4025: f64, t671: f64, t111: f64, t19449: f64, t2319: f64, t5449: f64, t1441: f64, t2363: f64, t2311: f64, t5456: f64, t1268: f64, t12725: f64, t12734: f64, t12739: f64, t12813: f64, t1458: f64, t19451: f64, t19456: f64, t19534: f64, t2314: f64, t26114: f64, t4028: f64, t4072: f64, t45632: f64, t5113: f64, t5493: f64, t55410: f64, t55568: f64, t7676: f64, t88: f64, t9348: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t55677 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2653(t25, t28, t45870, zeta_threshold);
        let t55709 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2654(t12633, t12636, t12648, t1426, t1434, t19331, t19334, t19335, t19338, t2252, t2255, t2283, t2304, t31, t3976, t4018, t5399, t5400, t5428, t5442, t55677, t628, t642, t65, t80);
        let t55716 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2655(t3961, t3966);
        let t55723 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2656(t3966);
        let t55751 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2657(t12606, t12705, t1420, t19378, t19381, t2262, t2267, t2274, t2275, t2278, t39, t39210, t3981, t43, t45970, t45974, t51, t5408, t5411, t5416, t55, t55677, t55716, t55723, t615);
        let t55801 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2658(t12606, t12695, t12699, t12702, t1420, t16558, t19368, t19369, t19372, t19377, t19390, t19397, t2244, t2250, t2267, t2274, t39, t39159, t39168, t3990, t51, t5392, t5398, t607, t615, t9287, t9300);
        let t55867 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2659(t12595, t12606, t12609, t12652, t16558, t19420, t19425, t19430, t19435, t2244, t2250, t2291, t2298, t39096, t39114, t4007, t4012, t5392, t5398, t55677, t55723, t607, t634, t638, t9321, t9330);
        let t55875 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2660(t12620, t12630, t12709, t1427, t1434, t19326, t19405, t19441, t2244, t2245, t2283, t2284, t2304, t33, t3998, t4018, t5392, t5393, t5427, t5442, t55723, t55751, t55801, t55867, t629, t642, t65, t66, t72, t80);
        let t55888 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2661(t19297, t604, t4021, t12571, t12585, t12588, t19299, t19310, t19318, t19445, t2235, t2240, t2241, t2307, t39054, t39063, t3958, t46104, t5389, t5445, t55631, t55673, t55709, t55875, t605, t645, t9228, t9231, t9239);
        let t55924 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2662(t2239, t5385, t12568, t12582, t12719, t1437, t16, t19313, t19445, t2240, t2241, t2307, t39033, t39037, t39043, t39049, t3953, t3958, t4021, t45844, t46099, t5389, t5445, t645, t86, t9231, t9239);
        let (t55927, t55934, t55943, t55946, t55962, t55967) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2663(t5, t55888, t55924, t112, t4025, t671, t111, t19449, t2319, t5449, t1441, t2363, t2311, t5456);
        let t55969 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2664(t1268, t12725, t12734, t12739, t12813, t1458, t19451, t19456, t19534, t2314, t2363, t26114, t4028, t4072, t45632, t5113, t5493, t55410, t55568, t55927, t55934, t55943, t55946, t55962, t55967, t671, t7676, t88, t9348);
    (t55677, t55716, t55723, t55927, t55934, t55943, t55946, t55962, t55967, t55969)
}
