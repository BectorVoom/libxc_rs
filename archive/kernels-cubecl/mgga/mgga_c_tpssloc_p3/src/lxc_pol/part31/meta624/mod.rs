//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta624 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1881;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta624<F: Float>(t22893: F, t28142: F, t80681: F, t28143: F, t80727: F, t28160: F, t6883: F, t19873: F, t26309: F, t19966: F, t6396: F, t80816: F) -> (F, F, F, F, F, F) {
        let (t97161, t97179, t97200, t97202, t97204, t97206) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1881::<F>(t22893, t28142, t80681, t28143, t80727, t28160, t6883, t19873, t26309, t19966, t6396, t80816);
    (t97161, t97179, t97200, t97202, t97204, t97206)
}
