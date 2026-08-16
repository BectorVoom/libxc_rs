//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 422/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk422(t184: f64, t2299: f64, t21: f64, t648: f64, t363: f64, t649: f64, t342: f64, t630: f64, t657: f64, t420: f64, t703: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2300 = t2299 * t184;
    let t2301 = t2300 * t21;
    let t2304 = t648 * t648;
    let t2305 = t2304 * t184;
    let t2306 = t2305 * t21;
    let t2309 = t649 * t363;
    let t2319 = t342 * t630 * t657 / 12.0_f64;
    let t2320 = t420 * t703;
    (t2300, t2301, t2304, t2305, t2306, t2309, t2319, t2320)
}
