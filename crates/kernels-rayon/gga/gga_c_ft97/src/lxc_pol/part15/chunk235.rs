//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 235/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk235(t370: f64, t942: f64, t27: f64, t89: f64, t354: f64, t923: f64, t348: f64) -> (f64, f64, f64, f64) {
    let t943 = t370 * t942;
    let t945 = t89 * t27 * t943;
    let t947 = -t354 - t923 / 18.0_f64 - t945 / 6.0_f64;
    let t948 = t348 * t947;
    (t943, t945, t947, t948)
}
