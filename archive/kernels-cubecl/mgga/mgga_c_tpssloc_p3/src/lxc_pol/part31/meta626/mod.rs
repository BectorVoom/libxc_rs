//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta626 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1883;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta626<F: Float>(t19894: F, t22833: F, t19886: F, t5293: F, t91100: F, t19991: F, t19882: F, t16311: F, t3788: F, t5286: F, t6936: F, t28101: F, t80958: F) -> (F, F, F, F, F, F, F) {
        let (t97223, t97225, t97227, t97229, t97231, t97236, t97238) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1883::<F>(t19894, t22833, t19886, t5293, t91100, t19991, t19882, t16311, t3788, t5286, t6936, t28101, t80958);
    (t97223, t97225, t97227, t97229, t97231, t97236, t97238)
}
