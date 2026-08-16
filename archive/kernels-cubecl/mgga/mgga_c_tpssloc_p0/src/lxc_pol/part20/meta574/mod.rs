//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta574 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2137;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta574<F: Float>(t3016: F, t698: F, t973: F, t10289: F, t2960: F, t10263: F, t2974: F, t10348: F, t135: F, t10352: F, t10232: F, t10208: F, t13822: F) -> (F, F, F, F, F, F, F) {
        let (t42914, t42916, t42918, t42925, t42936, t42944, t42951) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2137::<F>(t3016, t698, t973, t10289, t2960, t10263, t2974, t10348, t135, t10352, t10232, t10208, t13822);
    (t42914, t42916, t42918, t42925, t42936, t42944, t42951)
}
