//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta107 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk580;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta107<F: Float>(t1454: F, t626: F, t1453: F, t2331: F, t1444: F, t2341: F, t1449: F, t2349: F, t1409: F, t2433: F, t2440: F, t1472: F, t751: F) -> (F, F, F, F, F, F, F) {
        let (t4041, t4043, t4049, t4059, t4080, t4087, t4100) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk580::<F>(t1454, t626, t1453, t2331, t1444, t2341, t1449, t2349, t1409, t2433, t2440, t1472, t751);
    (t4041, t4043, t4049, t4059, t4080, t4087, t4100)
}
