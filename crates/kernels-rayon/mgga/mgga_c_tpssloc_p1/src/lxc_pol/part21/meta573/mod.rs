//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta573 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2288;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2289;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2290;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2291;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2292;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta573(t19404: f64, t33: f64, t5392: f64, t9321: f64, t2291: f64, t5398: f64, t9330: f64, t2298: f64, t16558: f64, t3966: f64, t4007: f64, t4012: f64, t607: f64, t634: f64, t638: f64, t72: f64, t1411: f64, t1427: f64, t1434: f64, t19363: f64, t3968: f64, t3971: f64, t3976: f64, t3998: f64, t4018: f64, t5428: f64, t5442: f64, t609: f64, t629: f64, t642: f64, t66: f64, t80: f64, t19356: f64, t12568: f64, t12571: f64, t1437: f64, t19297: f64, t19299: f64, t19310: f64, t19313: f64, t19318: f64, t2235: f64, t2240: f64, t3953: f64, t3958: f64, t4021: f64, t5389: f64, t5445: f64, t605: f64, t645: f64, t86: f64, t9231: f64, t9239: f64, t5: f64, t112: f64, t111: f64, t5449: f64, t1441: f64, t671: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19405, t19420, t19425, t19430, t19435, t19440) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2288(t19404, t33, t5392, t9321, t2291, t5398, t9330, t2298, t16558, t3966, t4007, t4012, t607, t634, t638);
        let (t19441, t19444) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2289(t19440, t72, t1411, t1427, t1434, t19363, t19405, t3968, t3971, t3976, t3998, t4018, t5428, t5442, t609, t629, t642, t66, t80);
        let (t19445, t19448) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2290(t19356, t19444, t12568, t12571, t1437, t19297, t19299, t19310, t19313, t19318, t2235, t2240, t3953, t3958, t4021, t5389, t5445, t605, t645, t86, t9231, t9239);
        let (t19449, t19450, t19451) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2291(t5, t19448, t112, t111, t5449);
        let t19456 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2292(t1441, t671);
    (t19405, t19420, t19425, t19430, t19435, t19441, t19445, t19449, t19450, t19451, t19456)
}
