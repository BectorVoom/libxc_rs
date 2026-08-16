//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta576 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1993;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta576<F: Float>(t22674: F, t22892: F, t22916: F, t22716: F, t6908: F, t22751: F, t22930: F, t22917: F, t22723: F, t22891: F, t22920: F, t117: F, t5247: F, t6559: F) -> (F, F, F, F, F, F, F) {
        let (t80659, t80663, t80665, t80667, t80670, t80671, t80681) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1993::<F>(t22674, t22892, t22916, t22716, t6908, t22751, t22930, t22917, t22723, t22891, t22920, t117, t5247, t6559);
    (t80659, t80663, t80665, t80667, t80670, t80671, t80681)
}
