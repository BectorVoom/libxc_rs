//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 149/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk149(t79: f64, t409: f64, t428: f64, t372: f64, t385: f64, t399: f64, t403: f64, t64: f64) -> f64 {
    let t80 = 0.1e-59_f64 < t79;
    let t429 = t409 * t428;
    let t432 = piecewise3(t80, -0.11627450473218896e-1_f64 * t372 * t385 + 2.0_f64 * t403 + 0.59273806478425129876e-2_f64 * t79 * t399 - t64 * t429, 0.0_f64);
    t432
}
