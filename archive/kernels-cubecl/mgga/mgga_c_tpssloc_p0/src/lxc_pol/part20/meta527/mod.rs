//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta527 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2061;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta527<F: Float>(t1369: F, t40059: F, t12345: F, t3876: F, t22843: F, t241: F, t67: F, t3872: F, t12353: F, t3866: F, t12339: F, t12211: F, t12375: F) -> (F, F, F, F, F, F, F) {
        let (t40060, t40065, t40070, t40079, t40081, t40083, t40089) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2061::<F>(t1369, t40059, t12345, t3876, t22843, t241, t67, t3872, t12353, t3866, t12339, t12211, t12375);
    (t40060, t40065, t40070, t40079, t40081, t40083, t40089)
}
