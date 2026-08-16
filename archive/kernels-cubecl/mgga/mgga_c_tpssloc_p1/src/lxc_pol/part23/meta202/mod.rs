//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta202 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk843;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta202<F: Float>(t10471: F, t3502: F, t11712: F, t3508: F, t6739: F, t1209: F, t475: F, t3639: F, t500: F, t1287: F, t2223: F, t1291: F, t9874: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t11887, t11888, t11889, t11913, t11914, t11915, t11947, t11982, t11984) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk843::<F>(t10471, t3502, t11712, t3508, t6739, t1209, t475, t3639, t500, t1287, t2223, t1291, t9874);
    (t11887, t11888, t11889, t11913, t11914, t11915, t11947, t11982, t11984)
}
