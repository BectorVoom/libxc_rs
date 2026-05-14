//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 55/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk55<F: Float>(t120: F, t126: F, t60: F) -> (F, F, F, F, F) {
    let t128 = 0.1247511874e1 - 0.859614445e0 * t120 + 0.812904345e0 * t126;
    let t129 = t128 * t128;
    let t130 = 0.56633563016285904186e-1 * t60;
    let t131 = 1.0 + t130;
    let t132 = t131 * t131;
    (t128, t129, t130, t131, t132)
}
