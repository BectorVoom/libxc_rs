//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta301 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1638;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1639;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1640;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta301<F: Float>(t3311: F, t419: F, t409: F, t11135: F, t10292: F, t281: F, t415: F, t1114: F, t2403: F, t3298: F, t699: F, t3301: F, t3304: F, t241: F, t3439: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11189, t11190) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1638::<F>(t3311, t419, t409);
        let (t11195, t11203, t11204, t11211) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1639::<F>(t11135, t10292, t281, t415, t1114, t2403);
        let (t11213, t11215, t11217, t11219) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1640::<F>(t3298, t699, t3301, t3304, t241, t3439);
    (t11189, t11190, t11195, t11203, t11204, t11211, t11213, t11215, t11217, t11219)
}
