//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta131 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk869;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk870;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk871;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk872;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta131(t3990: f64, t607: f64, t3966: f64, t55: f64, t1414: f64, t1420: f64, t2282: f64, t39: f64, t3982: f64, t3985: f64, t51: f64, t615: f64, t621: f64, t33: f64, t1409: f64, t2291: f64, t634: f64, t2298: f64, t638: f64, t72: f64, t1411: f64, t1427: f64, t1434: f64, t3962: f64, t3968: f64, t3971: f64, t3976: f64, t609: f64, t629: f64, t642: f64, t66: f64, t80: f64, t5: f64, t1437: f64, t2235: f64, t2240: f64, t3951: f64, t3953: f64, t3958: f64, t605: f64, t645: f64, t86: f64, t112: f64, t111: f64, t1441: f64, t671: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3991, t3994, t3997) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk869(t3990, t607, t3966, t55, t1414, t1420, t2282, t39, t3982, t3985, t51, t615, t621);
        let (t3998, t4007, t4012, t4018, t4021) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk870(t33, t3997, t1409, t2291, t3966, t634, t2298, t638, t607, t72, t1411, t1427, t1434, t3962, t3968, t3971, t3976, t609, t629, t642, t66, t80);
        let (t4025, t4026, t4028) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk871(t5, t1437, t2235, t2240, t3951, t3953, t3958, t4021, t605, t645, t86, t112, t111, t1441);
        let t4034 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk872(t671, t89);
    (t3991, t3994, t3997, t3998, t4007, t4012, t4018, t4021, t4025, t4026, t4028, t4034)
}
