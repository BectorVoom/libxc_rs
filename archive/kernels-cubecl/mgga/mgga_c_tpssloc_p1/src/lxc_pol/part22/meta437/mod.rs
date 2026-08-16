//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta437 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1778;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta437<F: Float>(t184: F, t19572: F, t17: F, t6320: F, t750: F, t1388: F, t1799: F, t15877: F, t11979: F, t15890: F, t15895: F, t588: F, t6328: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t19573, t19574, t19575, t19576, t19577, t19581, t19588, t19589, t19590, t19591) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1778::<F>(t184, t19572, t17, t6320, t750, t1388, t1799, t15877, t11979, t15890, t15895, t588, t6328);
    (t19573, t19574, t19575, t19576, t19577, t19581, t19588, t19589, t19590, t19591)
}
