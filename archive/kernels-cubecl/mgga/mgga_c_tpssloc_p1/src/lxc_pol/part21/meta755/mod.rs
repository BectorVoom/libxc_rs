//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta755 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2629;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta755<F: Float>(t1340: F, t53909: F, t16217: F, t3866: F, t1827: F, t39947: F, t16314: F, t16398: F, t16387: F, t40138: F, t5303: F, t12283: F, t16366: F) -> (F, F, F, F, F, F, F) {
        let (t54178, t54191, t54198, t54202, t54213, t54220, t54222) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2629::<F>(t1340, t53909, t16217, t3866, t1827, t39947, t16314, t16398, t16387, t40138, t5303, t12283, t16366);
    (t54178, t54191, t54198, t54202, t54213, t54220, t54222)
}
