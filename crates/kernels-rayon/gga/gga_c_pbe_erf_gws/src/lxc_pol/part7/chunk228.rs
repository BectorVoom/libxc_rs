//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 228/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk228(t573: f64, t606: f64, t25: f64, t575: f64, t599: f64, t604: f64) -> (f64, f64) {
    let t607 = t606 * t573;
    let t610 = -t599 - 0.35991666666666666667e-1_f64 * t575 - t604 - 0.66666666666666666667e-2_f64 * t25 * t607;
    (t607, t610)
}
