//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1090/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1090(t125390: f64, t125363: f64, t125366: f64, t125368: f64, t125370: f64, t125372: f64, t125374: f64, t125377: f64, t125379: f64, t125381: f64, t125383: f64, t125385: f64, t125387: f64, t125389: f64) -> f64 {
    let t125391 = 2.0_f64 * t125390;
    let t125392 = 4.0_f64 * t125363 + 4.0_f64 * t125366 + 4.0_f64 * t125368 + 4.0_f64 * t125370 + 4.0_f64 * t125372 + 4.0_f64 * t125374 + t125377 + t125379 + t125381 + t125383 + t125385 + t125387 + t125389 + t125391;
    t125392
}
