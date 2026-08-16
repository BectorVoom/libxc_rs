//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta393 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1198;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta393<F: Float>(t40281: F, t6396: F, t12345: F, t6427: F, t6431: F, t19815: F, t3865: F, t3789: F, t40159: F, t6390: F, t3798: F, t1827: F, t54532: F) -> (F, F, F, F, F, F, F, F) {
        let (t56993, t57011, t57019, t57021, t57033, t57041, t57056, t57073) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1198::<F>(t40281, t6396, t12345, t6427, t6431, t19815, t3865, t3789, t40159, t6390, t3798, t1827, t54532);
    (t56993, t57011, t57019, t57021, t57033, t57041, t57056, t57073)
}
