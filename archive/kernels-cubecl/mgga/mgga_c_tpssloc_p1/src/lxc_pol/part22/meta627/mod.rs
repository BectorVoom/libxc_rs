//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta627 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2162;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta627<F: Float>(t54047: F, t40167: F, t820: F, t16060: F, t3798: F, t12345: F, t5310: F, t1827: F, t40123: F, t3802: F, t39947: F, t1788: F, t9212: F) -> (F, F, F, F, F, F, F, F) {
        let (t54048, t54063, t54124, t54132, t54151, t54162, t54199, t54312) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2162::<F>(t54047, t40167, t820, t16060, t3798, t12345, t5310, t1827, t40123, t3802, t39947, t1788, t9212);
    (t54048, t54063, t54124, t54132, t54151, t54162, t54199, t54312)
}
