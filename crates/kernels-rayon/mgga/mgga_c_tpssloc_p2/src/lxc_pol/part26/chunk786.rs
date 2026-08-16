//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 786/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk786(t19: f64, t9223: f64, t9211: f64, t9213: f64, t9215: f64, t9217: f64, t9219: f64, t9221: f64, t2233: f64, t604: f64, t2239: f64, t601: f64) -> (f64, f64, f64) {
    let t9225 = 0.75936e3_f64 * t19 * t9223;
    let t9226 = -t9211 + t9213 - t9215 + t9217 - t9219 + t9221 - t9225;
    let t9228 = t2233 * t604;
    let t9231 = t601 * t2239;
    (t9226, t9228, t9231)
}
