//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta337 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1717;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1718;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1719;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1720;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta337<F: Float>(t12652: F, t65: F, t3961: F, t628: F, t12606: F, t31: F, t3967: F, t1409: F, t2244: F, t9287: F, t2267: F, t3966: F, t607: F, t2250: F, t3981: F, t43: F, t9300: F, t2274: F, t3990: F, t55: F, t1414: F, t1420: F, t2262: F, t2275: F, t2278: F, t39: F, t3982: F, t3985: F, t51: F, t615: F, t9311: F, t33: F, t12649: F, t1427: F, t1434: F, t2255: F, t2304: F, t3962: F, t3968: F, t3998: F, t4018: F, t609: F, t642: F, t80: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t12653, t12656, t12661, t12662, t12665, t12677, t12680) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1717::<F>(t12652, t65, t3961, t628, t12606, t31, t3967, t1409, t2244, t9287, t2267, t3966);
        let (t12681, t12684, t12687, t12695, t12699, t12702) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1718::<F>(t12680, t607, t2250, t3981, t12606, t43, t1409, t2244, t9300, t2274, t3966, t3990);
        let (t12705, t12708) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1719::<F>(t12606, t55, t12677, t12681, t12684, t12687, t12695, t12699, t12702, t1414, t1420, t2262, t2275, t2278, t39, t3982, t3985, t51, t615, t9311);
        let (t12709, t12718) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1720::<F>(t12708, t33, t12649, t12653, t12656, t12662, t12665, t1427, t1434, t2255, t2304, t3962, t3968, t3998, t4018, t609, t642, t80);
    (t12653, t12656, t12661, t12662, t12665, t12695, t12699, t12702, t12705, t12708, t12709, t12718)
}
