//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 141/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk141(t346: f64, t350: f64, t275: f64, t176: f64, sigma0: f64) -> (f64, f64) {
    let t352 = t346 * t350 - 0.18535714285714285714e-2_f64;
    let t353 = t352 * t275;
    let t355 = t176 * t353 * sigma0;
    (t352, t355)
}
