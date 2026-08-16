//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta242 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk973;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk974;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta242<F: Float>(t11228: F, t11268: F, t1118: F, t1099: F, t1097: F, t3311: F, t409: F, t3314: F, t422: F, t11191: F, t1146: F, t3399: F, t3402: F, t448: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11269, t11270, t11272, t11274, t11275, t11277, t11278, t11280, t11282) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk973::<F>(t11228, t11268, t1118, t1099, t1097, t3311, t409, t3314, t422, t11191, t1146, t3399);
        let t11285 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk974::<F>(t3402, t448);
    (t11269, t11270, t11272, t11274, t11275, t11277, t11278, t11280, t11282, t11285)
}
