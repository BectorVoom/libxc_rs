//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta311 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1111;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta311<F: Float>(t12083: F, t67: F, t758: F, t2505: F, t2527: F, t1294: F, t3691: F, t9905: F, t9892: F, t2368: F, t747: F, t9711: F) -> (F, F, F, F, F, F) {
        let (t39335, t39336, t39338, t39340, t39342, t39344) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1111::<F>(t12083, t67, t758, t2505, t2527, t1294, t3691, t9905, t9892, t2368, t747, t9711);
    (t39335, t39336, t39338, t39340, t39342, t39344)
}
