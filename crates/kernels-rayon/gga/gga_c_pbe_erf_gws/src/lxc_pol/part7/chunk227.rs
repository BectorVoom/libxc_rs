//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 227/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk227(t188: f64, t597: f64, t569: f64, t171: f64, t191: f64, t187: f64, t190: f64, t177: f64) -> (f64, f64, f64, f64, f64) {
    let t598 = t188 * t597;
    let t599 = 0.35991666666666666667e-1_f64 * t569;
    let t601 = t171 * t191;
    let t604 = 0.66666666666666666667e-2_f64 * t190 * t601 * t187;
    let t605 = 1.0_f64 / t177;
    let t606 = t191 * t605;
    (t598, t599, t601, t604, t606)
}
