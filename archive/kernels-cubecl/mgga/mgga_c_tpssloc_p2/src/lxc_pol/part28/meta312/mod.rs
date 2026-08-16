//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta312 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1237;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta312<F: Float>(t407: F, t11135: F, t410: F, t417: F, t1097: F, t3311: F, t409: F, t3314: F, t422: F, t1146: F, t3399: F, t3402: F, t448: F) -> (F, F, F, F, F, F, F) {
        let (t11243, t11247, t11265, t11275, t11277, t11282, t11285) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1237::<F>(t407, t11135, t410, t417, t1097, t3311, t409, t3314, t422, t1146, t3399, t3402, t448);
    (t11243, t11247, t11265, t11275, t11277, t11282, t11285)
}
