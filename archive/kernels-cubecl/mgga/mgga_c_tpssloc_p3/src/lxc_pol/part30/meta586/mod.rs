//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta586 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1965;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta586<F: Float>(t16752: F, t252: F, t5527: F, t828: F, t5611: F, t5584: F, t9975: F, t852: F, t17100: F, t225: F, t17087: F, t17060: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t58262, t58557, t58569, t58688, t58853, t59331, t59466, t59498, t59503) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1965::<F>(t16752, t252, t5527, t828, t5611, t5584, t9975, t852, t17100, t225, t17087, t17060);
    (t58262, t58557, t58569, t58688, t58853, t59331, t59466, t59498, t59503)
}
