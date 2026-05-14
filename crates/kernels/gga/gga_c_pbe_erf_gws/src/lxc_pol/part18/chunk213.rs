//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 213/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk213<F: Float>(t569: F, t171: F, t191: F, t187: F, t190: F, t177: F) -> (F, F, F, F) {
    let t599 = 0.35991666666666666667e-1 * t569;
    let t601 = t171 * t191;
    let t604 = 0.66666666666666666667e-2 * t190 * t601 * t187;
    let t605 = 1.0 / t177;
    let t606 = t191 * t605;
    (t599, t601, t604, t606)
}
