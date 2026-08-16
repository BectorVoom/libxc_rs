//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta524 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2058;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta524<F: Float>(t12283: F, t12413: F, t12267: F, t3802: F, t12279: F, t16398: F, t12409: F, t12167: F, t3792: F, t1314: F, t9569: F, t1329: F) -> (F, F, F, F, F, F, F) {
        let (t39973, t39975, t39983, t39989, t40000, t40005, t40006) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2058::<F>(t12283, t12413, t12267, t3802, t12279, t16398, t12409, t12167, t3792, t1314, t9569, t1329);
    (t39973, t39975, t39983, t39989, t40000, t40005, t40006)
}
