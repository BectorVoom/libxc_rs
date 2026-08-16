//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta429 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1265;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta429<F: Float>(t21788: F, t699: F, t21791: F, t21938: F, t3403: F, t21809: F, t3315: F, t21886: F, t3359: F, t1147: F, t21826: F, t1128: F, t21975: F) -> (F, F, F, F, F, F, F) {
        let (t71472, t71474, t71672, t71701, t71729, t71860, t71863) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1265::<F>(t21788, t699, t21791, t21938, t3403, t21809, t3315, t21886, t3359, t1147, t21826, t1128, t21975);
    (t71472, t71474, t71672, t71701, t71729, t71860, t71863)
}
