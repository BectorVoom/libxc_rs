//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 608/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk608<F: Float>(t8640: F, t895: F, t14: F, t7741: F, t12: F, t9: F) -> (F, F, F, F) {
    let t10921 = t8640 * t895;
    let t11174 = 1.0 / t14 / t7741;
    let t11175 = t12 * t11174;
    let t11176 = t9 * t11175;
    (t10921, t11174, t11175, t11176)
}
