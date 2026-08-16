//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta365 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1698;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1699;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1700;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1701;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1702;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1703;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta365(t12680: f64, t607: f64, t2250: f64, t3981: f64, t12606: f64, t43: f64, t1409: f64, t2244: f64, t9300: f64, t2274: f64, t3966: f64, t3990: f64, t55: f64, t12677: f64, t1414: f64, t1420: f64, t2262: f64, t2275: f64, t2278: f64, t39: f64, t3982: f64, t3985: f64, t51: f64, t615: f64, t9311: f64, t33: f64, t12649: f64, t12653: f64, t12656: f64, t12662: f64, t12665: f64, t1427: f64, t1434: f64, t2255: f64, t2304: f64, t3962: f64, t3968: f64, t3998: f64, t4018: f64, t609: f64, t642: f64, t80: f64, t12645: f64, t12566: f64, t12568: f64, t12571: f64, t12582: f64, t12585: f64, t12588: f64, t1437: f64, t2235: f64, t2240: f64, t2241: f64, t2307: f64, t3953: f64, t3958: f64, t4021: f64, t605: f64, t645: f64, t86: f64, t9228: f64, t9231: f64, t9239: f64, t5: f64, t112: f64, t111: f64, t4025: f64, t1441: f64, t2319: f64, t649: f64, t671: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12681, t12684, t12687, t12695, t12698, t12699, t12702) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1698(t12680, t607, t2250, t3981, t12606, t43, t1409, t2244, t9300, t2274, t3966, t3990);
        let t12708 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1699(t12606, t55, t12677, t12681, t12684, t12687, t12695, t12699, t12702, t1414, t1420, t2262, t2275, t2278, t39, t3982, t3985, t51, t615, t9311);
        let (t12709, t12718) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1700(t12708, t33, t12649, t12653, t12656, t12662, t12665, t1427, t1434, t2255, t2304, t3962, t3968, t3998, t4018, t609, t642, t80);
        let (t12719, t12722) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1701(t12645, t12718, t12566, t12568, t12571, t12582, t12585, t12588, t1437, t2235, t2240, t2241, t2307, t3953, t3958, t4021, t605, t645, t86, t9228, t9231, t9239);
        let (t12723, t12724, t12725) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1702(t5, t12722, t112, t111, t4025);
        let (t12728, t12734) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1703(t1441, t2319, t649, t671);
    (t12681, t12684, t12687, t12698, t12708, t12709, t12719, t12723, t12724, t12725, t12728, t12734)
}
