//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta642 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2182;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta642<F: Float>(t12283: F, t19976: F, t19886: F, t19815: F, t3802: F, t20000: F, t54566: F, t16398: F, t19873: F, t16397: F, t5234: F, t5252: F) -> (F, F, F, F, F, F) {
        let (t56837, t56853, t56878, t56883, t56885, t56888) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2182::<F>(t12283, t19976, t19886, t19815, t3802, t20000, t54566, t16398, t19873, t16397, t5234, t5252);
    (t56837, t56853, t56878, t56883, t56885, t56888)
}
