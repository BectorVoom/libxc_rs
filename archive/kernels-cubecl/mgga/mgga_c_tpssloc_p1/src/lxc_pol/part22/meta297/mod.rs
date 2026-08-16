//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta297 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1459;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta297<F: Float>(t2960: F, t4528: F, t1599: F, t698: F, t973: F, t135: F, t4542: F, t13552: F, t13550: F, t13644: F, t1036: F, t4622: F) -> (F, F, F, F, F, F, F, F) {
        let (t13907, t13908, t13909, t13915, t13921, t13922, t13923, t13946) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1459::<F>(t2960, t4528, t1599, t698, t973, t135, t4542, t13552, t13550, t13644, t1036, t4622);
    (t13907, t13908, t13909, t13915, t13921, t13922, t13923, t13946)
}
