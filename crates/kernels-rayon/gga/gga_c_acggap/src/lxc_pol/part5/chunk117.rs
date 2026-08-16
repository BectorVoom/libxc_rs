//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 117/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk117(t286: f64, t288: f64, t104: f64, t192: f64, t47: f64) -> (f64, f64, f64) {
    let t289 = t286 * t288;
    let t290 = 0.5848223622634646207e0_f64 * t289;
    let t291 = t104 * t192;
    let t292 = 1.0_f64 / t47;
    (t290, t291, t292)
}
