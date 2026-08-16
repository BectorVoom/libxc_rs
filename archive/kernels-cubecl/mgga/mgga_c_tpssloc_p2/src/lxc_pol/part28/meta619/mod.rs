//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta619 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1938;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1939;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta619<F: Float>(t22779: F, t26292: F, t1339: F, t54258: F, t550: F, t6936: F, t22827: F, t3788: F, t3792: F, t54068: F, t12289: F, t3791: F, t54014: F, t16311: F, t1825: F, t26288: F, t3734: F, t16314: F, t26309: F, t16227: F, t22833: F) -> (F, F, F, F, F, F, F, F) {
        let (t91225, t91229, t91233, t91237) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1938::<F>(t22779, t26292, t1339, t54258, t550, t6936, t22827, t3788, t3792, t54068, t12289, t3791, t54014);
        let (t91241, t91256, t91261, t91263) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1939::<F>(t16311, t3788, t3791, t6936, t1339, t1825, t26288, t3734, t16314, t26309, t16227, t22833);
    (t91225, t91229, t91233, t91237, t91241, t91256, t91261, t91263)
}
