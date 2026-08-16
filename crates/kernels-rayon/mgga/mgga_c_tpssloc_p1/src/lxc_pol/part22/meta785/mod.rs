//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta785 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2703;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2704;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2705;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2706;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2707;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2708;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2709;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2710;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2711;
use chunk9::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2712;
use chunk10::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2713;
use chunk11::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2714;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta785(t1410: f64, t1434: f64, t19335: f64, t19338: f64, t19343: f64, t19346: f64, t19349: f64, t19404: f64, t20227: f64, t3961: f64, t3967: f64, t4018: f64, t5400: f64, t5403: f64, t5427: f64, t642: f64, t80: f64, t12680: f64, t1420: f64, t16558: f64, t19368: f64, t19390: f64, t19391: f64, t19394: f64, t19398: f64, t20217: f64, t20234: f64, t2267: f64, t39: f64, t39159: f64, t3966: f64, t3981: f64, t3991: f64, t45970: f64, t45974: f64, t51: f64, t5398: f64, t5416: f64, t607: f64, t68513: f64, t12698: f64, t19401: f64, t20235: f64, t20238: f64, t20241: f64, t20246: f64, t2274: f64, t39168: f64, t39210: f64, t3990: f64, t3994: f64, t43: f64, t55: f64, t615: f64, t621: f64, t67060: f64, t12595: f64, t12598: f64, t12609: f64, t12612: f64, t17635: f64, t19420: f64, t19430: f64, t2291: f64, t2298: f64, t39096: f64, t39114: f64, t4007: f64, t4012: f64, t634: f64, t638: f64, t1427: f64, t19326: f64, t19405: f64, t19441: f64, t20210: f64, t20265: f64, t20285: f64, t33: f64, t3997: f64, t3998: f64, t5392: f64, t5393: f64, t5428: f64, t5442: f64, t629: f64, t66: f64, t72: f64, t1437: f64, t19445: f64, t20201: f64, t20204: f64, t20288: f64, t2235: f64, t2240: f64, t39054: f64, t39063: f64, t3953: f64, t4021: f64, t5389: f64, t5445: f64, t605: f64, t645: f64, t75356: f64, t75392: f64, t9231: f64, t9239: f64, t5: f64, t12568: f64, t12571: f64, t19299: f64, t19310: f64, t19313: f64, t19318: f64, t39043: f64, t3958: f64, t45844: f64, t46085: f64, t46086: f64, t46087: f64, t46088: f64, t46089: f64, t46090: f64, t46104: f64, t55880: f64, t55921: f64, t75284: f64, t86: f64, t112: f64, t5449: f64, t671: f64, t20305: f64, t626: f64, t20308: f64, t1453: f64, t5488: f64, t20343: f64, t1444: f64, t5396: f64, t16: f64, t39031: f64, t12774: f64, t19503: f64, t2: f64, t20311: f64, t20312: f64, t20315: f64, t20318: f64, t20319: f64, t20322: f64, t2219: f64, t2341: f64, t4049: f64, t4060: f64, t45496: f64, t45697: f64, t5468: f64, t5475: f64, t584: f64, t657: f64, t659: f64, t663: f64, t92: f64, t95: f64, t1449: f64, t5484: f64, t100: f64, t103: f64, t12795: f64, t1447: f64, t19514: f64, t19518: f64, t19522: f64, t19525: f64, t19526: f64, t20331: f64, t20338: f64, t2349: f64, t4059: f64, t4064: f64, t45460: f64, t45707: f64, t5480: f64, t55491: f64, t662: f64, t12757: f64, t19473: f64, t19529: f64, t20304: f64, t20342: f64, t2331: f64, t29903: f64, t4043: f64, t4067: f64, t45435: f64, t55420: f64, t64: f64, t656: f64, t666: f64, t109: f64, t45421: f64, t45422: f64, t45656: f64, t45659: f64, t45689: f64, t55531: f64, t55537: f64, t55546: f64, t55559: f64, t55561: f64, t1268: f64, t12725: f64, t1458: f64, t19451: f64, t19456: f64, t19534: f64, t20347: f64, t2314: f64, t26114: f64, t26117: f64, t28002: f64, t4028: f64, t4072: f64, t5113: f64, t5493: f64, t55943: f64, t67001: f64, t75275: f64, t7676: f64) -> (f64, f64, f64, f64) {
        let t75419 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2703(t1410, t1434, t19335, t19338, t19343, t19346, t19349, t19404, t20227, t3961, t3967, t4018, t5400, t5403, t5427, t642, t80);
        let t75461 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2704(t12680, t1420, t16558, t19368, t19390, t19391, t19394, t19398, t20217, t20234, t2267, t39, t39159, t3966, t3981, t3991, t45970, t45974, t51, t5398, t5416, t607, t68513);
        let t75494 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2705(t12698, t1420, t16558, t19401, t20217, t20234, t20235, t20238, t20241, t20246, t2274, t39, t39168, t39210, t3990, t3994, t43, t51, t5398, t5416, t55, t607, t615, t621, t67060);
        let t75543 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2706(t12595, t12598, t12609, t12612, t16558, t17635, t19420, t19430, t20217, t20234, t2291, t2298, t39096, t39114, t3966, t4007, t4012, t5398, t607, t634, t638, t67060);
        let t75547 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2707(t1427, t1434, t19326, t19405, t19441, t20210, t20265, t20285, t33, t3997, t3998, t4018, t5392, t5393, t5428, t5442, t629, t642, t66, t72, t75461, t75494, t75543, t80);
        let t75552 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2708(t1437, t19445, t20201, t20204, t20288, t2235, t2240, t39054, t39063, t3953, t4021, t5389, t5445, t605, t645, t75356, t75392, t75419, t75547, t9231, t9239);
        let t75554 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2709(t5, t12568, t12571, t1437, t19299, t19310, t19313, t19318, t39043, t3958, t4021, t45844, t46085, t46086, t46087, t46088, t46089, t46090, t46104, t5389, t5445, t55880, t55921, t645, t75284, t75552, t86);
        let (t75555, t75560, t75592, t75601, t75603, t75613, t75631) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2710(t112, t75554, t5449, t671, t20305, t626, t20308, t1453, t5488, t20343, t1444, t5396);
        let (t75649, t75657) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2711(t16, t39031, t12774, t19503, t2, t20311, t20312, t20315, t20318, t20319, t20322, t2219, t2341, t4049, t4060, t45496, t45697, t5396, t5468, t5475, t584, t657, t659, t663, t75631, t92, t95);
        let t75694 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2712(t1449, t5484, t100, t103, t12795, t1447, t19514, t19518, t19522, t19525, t19526, t2, t20331, t20338, t2219, t2349, t4059, t4064, t45460, t45707, t5475, t5480, t55491, t584, t662, t75649);
        let t75699 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2713(t12757, t19473, t19529, t20304, t20342, t2331, t29903, t4043, t4067, t45435, t5488, t55420, t64, t656, t666, t75592, t75601, t75603, t75613, t75657, t75694);
        let (t75701, t75704) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2714(t109, t45421, t45422, t45656, t45659, t45689, t55531, t55537, t55546, t55559, t55561, t75699, t1268, t12725, t1458, t19451, t19456, t19534, t20347, t2314, t26114, t26117, t28002, t4028, t4072, t5113, t5493, t55943, t67001, t671, t75275, t75555, t75560, t7676);
    (t75555, t75560, t75701, t75704)
}
