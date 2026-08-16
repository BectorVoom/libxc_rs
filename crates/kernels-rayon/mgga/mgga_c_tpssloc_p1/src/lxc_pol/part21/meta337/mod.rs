//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta337 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1717;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1718;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1719;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1720;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta337(t12652: f64, t65: f64, t3961: f64, t628: f64, t12606: f64, t31: f64, t3967: f64, t1409: f64, t2244: f64, t9287: f64, t2267: f64, t3966: f64, t607: f64, t2250: f64, t3981: f64, t43: f64, t9300: f64, t2274: f64, t3990: f64, t55: f64, t1414: f64, t1420: f64, t2262: f64, t2275: f64, t2278: f64, t39: f64, t3982: f64, t3985: f64, t51: f64, t615: f64, t9311: f64, t33: f64, t12649: f64, t1427: f64, t1434: f64, t2255: f64, t2304: f64, t3962: f64, t3968: f64, t3998: f64, t4018: f64, t609: f64, t642: f64, t80: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12653, t12656, t12661, t12662, t12665, t12677, t12680) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1717(t12652, t65, t3961, t628, t12606, t31, t3967, t1409, t2244, t9287, t2267, t3966);
        let (t12681, t12684, t12687, t12695, t12699, t12702) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1718(t12680, t607, t2250, t3981, t12606, t43, t1409, t2244, t9300, t2274, t3966, t3990);
        let (t12705, t12708) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1719(t12606, t55, t12677, t12681, t12684, t12687, t12695, t12699, t12702, t1414, t1420, t2262, t2275, t2278, t39, t3982, t3985, t51, t615, t9311);
        let (t12709, t12718) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1720(t12708, t33, t12649, t12653, t12656, t12662, t12665, t1427, t1434, t2255, t2304, t3962, t3968, t3998, t4018, t609, t642, t80);
    (t12653, t12656, t12661, t12662, t12665, t12695, t12699, t12702, t12705, t12708, t12709, t12718)
}
