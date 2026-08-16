//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta610 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2138;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta610<F: Float>(t11153: F, t1229: F, t3242: F, t486: F, t11147: F, t3584: F, t2403: F, t4775: F, t4772: F, t50826: F, t50919: F, t50948: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t50992, t50998, t51002, t51039, t51040, t51051, t51058, t51073, t51082) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2138::<F>(t11153, t1229, t3242, t486, t11147, t3584, t2403, t4775, t4772, t50826, t50919, t50948);
    (t50992, t50998, t51002, t51039, t51040, t51051, t51058, t51073, t51082)
}
