//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta463 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1789;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1790;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta463<F: Float>(t22893: F, t6639: F, t23164: F, t6546: F, t6551: F, t6640: F, t22641: F, t2587: F) -> (F, F, F, F, F, F, F) {
        let (t23165, t23166, t23167, t23168) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1789::<F>(t22893, t6639, t23164, t6546, t6551);
        let (t23169, t23170, t23171) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1790::<F>(t23168, t6640, t22641, t2587);
    (t23165, t23166, t23167, t23168, t23169, t23170, t23171)
}
