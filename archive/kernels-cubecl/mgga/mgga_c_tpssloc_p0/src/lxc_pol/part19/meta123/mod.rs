//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta123 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk665;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk666;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta123<F: Float>(t300: F, t3407: F, t3369: F, t1143: F, t1166: F, t1156: F, t3375: F, t3377: F, t1164: F, t1147: F, t3395: F, t3400: F, t3403: F, t457: F, t697: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3408, t3410, t3411) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk665::<F>(t300, t3407, t3369, t1143);
        let (t3413, t3415, t3417, t3419, t3421, t3422, t3423, t3425, t3426) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk666::<F>(t1166, t3411, t1156, t3375, t3377, t1164, t1147, t3395, t3400, t3403, t457, t697);
    (t3408, t3410, t3411, t3413, t3415, t3417, t3419, t3421, t3422, t3423, t3425, t3426)
}
