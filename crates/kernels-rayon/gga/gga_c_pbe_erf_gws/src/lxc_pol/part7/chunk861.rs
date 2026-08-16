//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 861/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk861(t5907: f64, t679: f64, t5904: f64, t666: f64, t16553: f64, t16556: f64, t16557: f64, t16561: f64, t16566: f64, t16567: f64, t16572: f64, t16574: f64, t16580: f64, t225: f64, t231: f64) -> f64 {
    let t16584 = t5907 * t679;
    let t16586 = t666 * t5904;
    let t16588 = t16553 + t16556 + 0.86568330898918747016e0_f64 * t16557 - t16561 + t16566 + 0.13418091289332405787e0_f64 * t16567 + t16572 + t16574 + 4.0_f64 / 3.0_f64 * t16580 * t225 * t231 + 16.0_f64 / 3.0_f64 * t16584 + 16.0_f64 / 3.0_f64 * t16586;
    t16588
}
