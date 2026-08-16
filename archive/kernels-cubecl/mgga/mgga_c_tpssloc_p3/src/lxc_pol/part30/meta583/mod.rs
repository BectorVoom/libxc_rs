//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta583 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1962;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta583<F: Float>(t2239: F, t5385: F, t111: F, t19449: F, t19644: F, t225: F, t20038: F, t20032: F, t20040: F, t19635: F, t20048: F, t1351: F, t6414: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t55921, t55943, t56422, t56434, t56580, t56596, t56607, t56640, t56812) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1962::<F>(t2239, t5385, t111, t19449, t19644, t225, t20038, t20032, t20040, t19635, t20048, t1351, t6414);
    (t55921, t55943, t56422, t56434, t56580, t56596, t56607, t56640, t56812)
}
