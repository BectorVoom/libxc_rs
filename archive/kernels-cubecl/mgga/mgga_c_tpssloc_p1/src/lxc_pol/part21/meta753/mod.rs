//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta753 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2627;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta753<F: Float>(t12283: F, t16248: F, t40138: F, t5293: F, t16275: F, t16271: F, t16383: F, t16370: F, t16060: F, t3798: F, t1354: F, t12345: F, t5310: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t54088, t54090, t54092, t54114, t54116, t54118, t54124, t54125, t54131) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2627::<F>(t12283, t16248, t40138, t5293, t16275, t16271, t16383, t16370, t16060, t3798, t1354, t12345, t5310);
    (t54088, t54090, t54092, t54114, t54116, t54118, t54124, t54125, t54131)
}
