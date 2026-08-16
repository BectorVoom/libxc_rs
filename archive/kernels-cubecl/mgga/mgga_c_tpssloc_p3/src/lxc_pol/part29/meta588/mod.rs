//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta588 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2012;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta588<F: Float>(t22700: F, t6914: F, t22699: F, t22704: F, t22705: F, t22741: F, t22696: F, t3879: F, t552: F, t22747: F, t22893: F, t80681: F) -> (F, F, F, F, F, F) {
        let (t81099, t81115, t81125, t81127, t81129, t81140) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2012::<F>(t22700, t6914, t22699, t22704, t22705, t22741, t22696, t3879, t552, t22747, t22893, t80681);
    (t81099, t81115, t81125, t81127, t81129, t81140)
}
