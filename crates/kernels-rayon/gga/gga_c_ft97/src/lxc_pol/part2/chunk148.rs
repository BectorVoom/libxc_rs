//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 148/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk148(t370: f64, t432: f64, t27: f64, t89: f64, t354: f64, t366: f64, t348: f64) -> (f64, f64, f64, f64) {
    let t433 = t370 * t432;
    let t435 = t89 * t27 * t433;
    let t437 = -t354 - t366 / 18.0_f64 - t435 / 6.0_f64;
    let t438 = t348 * t437;
    (t433, t435, t437, t438)
}
