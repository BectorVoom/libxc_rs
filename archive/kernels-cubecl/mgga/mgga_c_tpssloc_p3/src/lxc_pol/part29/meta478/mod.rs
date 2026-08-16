//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta478 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1816;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta478<F: Float>(t491: F, t7319: F, t7287: F, t3439: F, t461: F, t3243: F, t7286: F, t225: F) -> (F, F, F, F, F, F, F) {
        let (t24590, t24591, t24594, t24596, t24597, t24600, t24601) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1816::<F>(t491, t7319, t7287, t3439, t461, t3243, t7286, t225);
    (t24590, t24591, t24594, t24596, t24597, t24600, t24601)
}
