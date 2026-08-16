//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta217 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk915;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta217<F: Float>(t10680: F, t10695: F, t913: F, t893: F, t2840: F, t891: F, t275: F, t2843: F, t290: F, t10662: F, t10524: F, t2929: F, t951: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t10696, t10697, t10699, t10701, t10702, t10704, t10705, t10707, t10709) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk915::<F>(t10680, t10695, t913, t893, t2840, t891, t275, t2843, t290, t10662, t10524, t2929, t951);
    (t10696, t10697, t10699, t10701, t10702, t10704, t10705, t10707, t10709)
}
