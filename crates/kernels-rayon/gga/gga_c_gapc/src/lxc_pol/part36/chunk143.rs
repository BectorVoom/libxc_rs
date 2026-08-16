//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 143/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk143(t144: f64, t474: f64, t4: f64, t437: f64, t22: f64, t413: f64) -> (f64, f64, f64, f64) {
    let t475 = t474 * t144;
    let t476 = t437 * t4;
    let t477 = t475 * t476;
    let t481 = 1.0_f64 / t22 / t413;
    (t475, t476, t477, t481)
}
