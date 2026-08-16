//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta44 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk303;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta44<F: Float>(t291: F, t888: F, t287: F, t275: F, t276: F, t880: F, t886: F) -> (F, F, F, F, F, F) {
        let (t890, t891, t892, t893, t894, t896) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk303::<F>(t291, t888, t287, t275, t276, t880, t886);
    (t890, t891, t892, t893, t894, t896)
}
