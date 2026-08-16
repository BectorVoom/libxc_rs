//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1415/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1415(t35527: f64, t35536: f64, t36375: f64, t36376: f64, t36377: f64, t36378: f64, t36379: f64, t36380: f64, t36381: f64, t36383: f64, t36384: f64, t36386: f64, t36387: f64, t36388: f64) -> f64 {
    let t38548 = t36375 + t36376 - t36377 + t36378 - t36379 + t36380 - t36381 - 0.5431140175846100239e-5_f64 * t35527 - t36383 - t36384 + 0.5431140175846100239e-5_f64 * t35536 + t36386 + t36387 - t36388;
    t38548
}
