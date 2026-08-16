//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta336 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1531;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta336<F: Float>(t16562: F, t16574: F, t145: F, t185: F, t5520: F, t751: F, t157: F, t182: F, t12861: F, t4119: F, t4315: F, t5392: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t16575, t16576, t16577, t16578, t16579, t16581, t16582, t16583, t16586) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1531::<F>(t16562, t16574, t145, t185, t5520, t751, t157, t182, t12861, t4119, t4315, t5392);
    (t16575, t16576, t16577, t16578, t16579, t16581, t16582, t16583, t16586)
}
