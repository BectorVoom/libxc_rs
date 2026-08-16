//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta239 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk969;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta239<F: Float>(t11203: F, t1113: F, t11163: F, t136: F, t11172: F, t1114: F, t2403: F, t3298: F, t699: F, t3301: F, t3304: F, t241: F, t3439: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11204, t11205, t11206, t11208, t11209, t11211, t11213, t11215, t11217, t11219) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk969::<F>(t11203, t1113, t11163, t136, t11172, t1114, t2403, t3298, t699, t3301, t3304, t241, t3439);
    (t11204, t11205, t11206, t11208, t11209, t11211, t11213, t11215, t11217, t11219)
}
