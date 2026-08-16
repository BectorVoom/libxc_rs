//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta510 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2026;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta510<F: Float>(t2505: F, t2527: F, t1294: F, t3691: F, t9905: F, t9892: F, t2368: F, t747: F, t9711: F, t9810: F, t9844: F, t39321: F) -> (F, F, F, F, F, F, F, F) {
        let (t39336, t39338, t39339, t39341, t39344, t39346, t39347, t39349) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2026::<F>(t2505, t2527, t1294, t3691, t9905, t9892, t2368, t747, t9711, t9810, t9844, t39321);
    (t39336, t39338, t39339, t39341, t39344, t39346, t39347, t39349)
}
