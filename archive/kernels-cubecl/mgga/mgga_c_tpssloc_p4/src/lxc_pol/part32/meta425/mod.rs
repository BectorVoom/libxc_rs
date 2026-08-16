//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta425 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1636;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1637;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1638;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1639;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1640;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1641;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1642;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1643;
use chunk8::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1644;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta425<F: Float>(t1437: F, t4021: F, t5445: F, t645: F, t1409: F, t65: F, t67: F, t1864: F, t3966: F, t5392: F, t628: F, t17635: F, t16558: F, t31: F, t5399: F, t1426: F, t3961: F, t3967: F, t1410: F, t3997: F, t1434: F, t3962: F, t5393: F, t5400: F, t5403: F, t642: F, t80: F, t5427: F, t608: F, t9287: F, t607: F, t3981: F, t2267: F, t5398: F, t43: F, t9300: F, t3990: F, t2274: F, t55: F, t1420: F, t39: F, t3991: F, t3994: F, t51: F, t5408: F, t5411: F, t5416: F, t615: F, t621: F, t9311: F, t33: F, t9321: F, t2291: F, t9330: F, t2298: F, t4007: F, t4012: F, t634: F, t638: F, t72: F, t1411: F, t1427: F, t3968: F, t3971: F, t3976: F, t3998: F, t4018: F, t5428: F, t5442: F, t609: F, t629: F, t66: F, t12568: F, t12571: F, t19297: F, t19299: F, t19310: F, t2235: F, t2240: F, t3953: F, t3958: F, t5389: F, t605: F, t86: F, t9231: F, t9239: F, t5: F, t112: F, t111: F, t5449: F, t1441: F, t671: F) -> (F, F, F, F, F, F, F) {
        let (t19313, t19318, t19322, t19323, t19326, t19331) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1636::<F>(t1437, t4021, t5445, t645, t1409, t65, t67, t1864, t3966, t5392, t628, t17635);
        let (t19334, t19356) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1637::<F>(t16558, t31, t65, t5399, t628, t1426, t3961, t3967, t1410, t3997, t1434, t19322, t19323, t19326, t19331, t3962, t5393, t5400, t5403, t642, t80);
        let (t19363, t19369, t19372, t19378, t19381, t19390) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1638::<F>(t5427, t608, t5392, t9287, t607, t3966, t3981, t2267, t5398, t16558, t43, t9300);
        let t19404 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1639::<F>(t19390, t607, t3966, t3990, t2274, t5398, t16558, t55, t1420, t19369, t19372, t19378, t19381, t39, t3991, t3994, t51, t5408, t5411, t5416, t615, t621, t9311);
        let (t19405, t19440) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1640::<F>(t19404, t33, t5392, t9321, t2291, t5398, t9330, t2298, t16558, t3966, t4007, t4012, t607, t634, t638);
        let t19444 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1641::<F>(t19440, t72, t1411, t1427, t1434, t19363, t19405, t3968, t3971, t3976, t3998, t4018, t5428, t5442, t609, t629, t642, t66, t80);
        let (t19445, t19448) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1642::<F>(t19356, t19444, t12568, t12571, t1437, t19297, t19299, t19310, t19313, t19318, t2235, t2240, t3953, t3958, t4021, t5389, t5445, t605, t645, t86, t9231, t9239);
        let (t19449, t19450, t19451) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1643::<F>(t5, t19448, t112, t111, t5449);
        let t19456 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1644::<F>(t1441, t671);
    (t19334, t19440, t19445, t19449, t19450, t19451, t19456)
}
