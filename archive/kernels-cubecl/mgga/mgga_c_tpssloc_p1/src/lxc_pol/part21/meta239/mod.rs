//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta239 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1416;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1417;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1418;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1419;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta239<F: Float>(t1020: F, t1041: F, t1618: F, t1622: F, t3039: F, t3070: F, t3084: F, t3130: F, t3160: F, t378: F, t4572: F, t4604: F, t4625: F, t4631: F, t4641: F, t4644: F, t5857: F, t5861: F, t5869: F, t5875: F, t5880: F, t5885: F, t5890: F, t5894: F, t5900: F, t5905: F, t5909: F, t973: F, t349: F, t1634: F, t3174: F, t381: F, t5872: F, t3188: F, t1615: F, t1625: F, t1060: F, t5866: F) -> (F, F, F, F, F, F, F, F, F) {
        let t5914 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1416::<F>(t1020, t1041, t1618, t1622, t3039, t3070, t3084, t3130, t3160, t378, t4572, t4604, t4625, t4631, t4641, t4644, t5857, t5861, t5869, t5875, t5880, t5885, t5890, t5894, t5900, t5905, t5909, t973);
        let (t5915, t5919, t5920, t5928) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1417::<F>(t349, t5914, t1634, t3174, t381, t5872);
        let (t5929, t5932) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1418::<F>(t3188, t5928, t1615, t1625);
        let (t5933, t5936) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1419::<F>(t1060, t5932, t381, t5866);
    (t5914, t5915, t5919, t5920, t5928, t5929, t5932, t5933, t5936)
}
