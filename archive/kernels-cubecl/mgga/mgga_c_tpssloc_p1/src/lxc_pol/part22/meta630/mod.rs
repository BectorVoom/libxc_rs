//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta630 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2165;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta630<F: Float>(t54428: F, t39571: F, t39581: F, t2225: F, t5168: F, t5154: F, t9892: F, t39601: F, t39605: F, t39607: F, t39609: F, t39634: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t54429, t54430, t54431, t54432, t54434, t54436, t54437, t54438, t54439, t54447) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2165::<F>(t54428, t39571, t39581, t2225, t5168, t5154, t9892, t39601, t39605, t39607, t39609, t39634);
    (t54429, t54430, t54431, t54432, t54434, t54436, t54437, t54438, t54439, t54447)
}
