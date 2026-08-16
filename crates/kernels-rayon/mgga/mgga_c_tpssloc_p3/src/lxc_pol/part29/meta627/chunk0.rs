//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2070/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2070(t11841: f64, t7310: f64, t11791: f64, t7345: f64, t11820: f64, t7339: f64, t11698: f64, t24741: f64, t2132: f64, t24746: f64, t86202: f64, t11754: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t86343 = t7310 * t11841;
    let t86348 = t7345 * t11791;
    let t86350 = t7339 * t11820;
    let t86354 = t24741 * t11698;
    let t86357 = t2132 * t86202 * t24746;
    let t86365 = t7310 * t11754;
    (t86343, t86348, t86350, t86354, t86357, t86365)
}
