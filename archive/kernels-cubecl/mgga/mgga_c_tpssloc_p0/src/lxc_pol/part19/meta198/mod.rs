//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta198 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk865;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta198<F: Float>(t2960: F, t2971: F, t2970: F, t2995: F, t973: F, t2769: F, t40: F) -> (F, F, F, F, F) {
        let (t10267, t10273, t10274, t10276, t10277) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk865::<F>(t2960, t2971, t2970, t2995, t973, t2769, t40);
    (t10267, t10273, t10274, t10276, t10277)
}
