//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta573 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2288;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2289;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2290;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2291;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2292;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta573<F: Float>(t19404: F, t33: F, t5392: F, t9321: F, t2291: F, t5398: F, t9330: F, t2298: F, t16558: F, t3966: F, t4007: F, t4012: F, t607: F, t634: F, t638: F, t72: F, t1411: F, t1427: F, t1434: F, t19363: F, t3968: F, t3971: F, t3976: F, t3998: F, t4018: F, t5428: F, t5442: F, t609: F, t629: F, t642: F, t66: F, t80: F, t19356: F, t12568: F, t12571: F, t1437: F, t19297: F, t19299: F, t19310: F, t19313: F, t19318: F, t2235: F, t2240: F, t3953: F, t3958: F, t4021: F, t5389: F, t5445: F, t605: F, t645: F, t86: F, t9231: F, t9239: F, t5: F, t112: F, t111: F, t5449: F, t1441: F, t671: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t19405, t19420, t19425, t19430, t19435, t19440) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2288::<F>(t19404, t33, t5392, t9321, t2291, t5398, t9330, t2298, t16558, t3966, t4007, t4012, t607, t634, t638);
        let (t19441, t19444) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2289::<F>(t19440, t72, t1411, t1427, t1434, t19363, t19405, t3968, t3971, t3976, t3998, t4018, t5428, t5442, t609, t629, t642, t66, t80);
        let (t19445, t19448) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2290::<F>(t19356, t19444, t12568, t12571, t1437, t19297, t19299, t19310, t19313, t19318, t2235, t2240, t3953, t3958, t4021, t5389, t5445, t605, t645, t86, t9231, t9239);
        let (t19449, t19450, t19451) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2291::<F>(t5, t19448, t112, t111, t5449);
        let t19456 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2292::<F>(t1441, t671);
    (t19405, t19420, t19425, t19430, t19435, t19441, t19445, t19449, t19450, t19451, t19456)
}
