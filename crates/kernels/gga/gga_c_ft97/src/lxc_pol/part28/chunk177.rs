//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 177/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk177<F: Float>(t1053: F, t605: F, t144: F, t1026: F, t1030: F, t1047: F, t28: F, t446: F, t568: F, t89: F) -> (F, F) {
    let t1054 = t605 * t1053;
    let t1055 = t144 * t1054;
    let t1058 = -t568 - t446 * t1026 / F::new(9.0) - t446 * t1030 / F::new(3.0) + t89 * t28 * t1047 / F::new(3.0) - t446 * t1055 / F::new(3.0);
    (t1055, t1058)
}
