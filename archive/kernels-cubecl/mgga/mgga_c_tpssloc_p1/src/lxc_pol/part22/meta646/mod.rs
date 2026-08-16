//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta646 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2186;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta646<F: Float>(t40159: F, t6390: F, t19815: F, t3798: F, t1354: F, t40130: F, t1827: F, t54532: F, t16232: F, t5234: F, t1351: F, t6387: F) -> (F, F, F, F, F, F, F) {
        let (t57041, t57056, t57057, t57071, t57073, t57081, t57091) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2186::<F>(t40159, t6390, t19815, t3798, t1354, t40130, t1827, t54532, t16232, t5234, t1351, t6387);
    (t57041, t57056, t57057, t57071, t57073, t57081, t57091)
}
