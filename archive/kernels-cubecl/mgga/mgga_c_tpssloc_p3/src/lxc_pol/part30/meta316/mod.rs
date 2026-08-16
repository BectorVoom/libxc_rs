//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta316 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1340;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta316<F: Float>(t3314: F, t422: F, t1146: F, t3399: F, t3402: F, t448: F, t445: F, t1143: F, t3375: F, t1124: F, t3331: F, t440: F) -> (F, F, F, F, F, F, F) {
        let (t11277, t11282, t11285, t11292, t11297, t11303, t11310) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1340::<F>(t3314, t422, t1146, t3399, t3402, t448, t445, t1143, t3375, t1124, t3331, t440);
    (t11277, t11282, t11285, t11292, t11297, t11303, t11310)
}
