//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta264 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1407;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta264<F: Float>(t475: F, t6739: F, t3639: F, t500: F, t1285: F, t2223: F, t1287: F, t1291: F, t9874: F, t25: F, t514: F, t28: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t11915, t11947, t11979, t11981, t11982, t11984, t11985, t11987, t11998) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1407::<F>(t475, t6739, t3639, t500, t1285, t2223, t1287, t1291, t9874, t25, t514, t28);
    (t11915, t11947, t11979, t11981, t11982, t11984, t11985, t11987, t11998)
}
