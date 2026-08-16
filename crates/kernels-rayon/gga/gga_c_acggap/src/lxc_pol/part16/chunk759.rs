//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 759/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk759(t1083: f64, t1089: f64, t8484: f64, t598: f64, t8489: f64, t7458: f64, t1980: f64, t1988: f64, t2299: f64, t1530: f64, t7646: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8502 = t1089 * t1083 * t8484;
    let t8503 = t598 * t8502;
    let t8505 = t1083 * t8489;
    let t8506 = t7458 * t8505;
    let t8507 = t1980 * t8506;
    let t8509 = t1988 * t2299;
    let t8511 = t1530 * t7646;
    (t8502, t8503, t8505, t8506, t8507, t8509, t8511)
}
