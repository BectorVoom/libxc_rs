//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 679/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk679(t301: f64, t7381: f64, t7380: f64, t1983: f64, t372: f64, t2095: f64, t355: f64, t429: f64, t1017: f64, t604: f64, t336: f64, t578: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7382 = t7381 * t301;
    let t7383 = t7380 * t7382;
    let t7384 = t7383 / 32.0_f64;
    let t7386 = t1983 * t372;
    let t7387 = t2095 * t7386;
    let t7388 = t7387 / 96.0_f64;
    let t7389 = t429 * t355;
    let t7390 = t2095 * t7389;
    let t7391 = 0.1528125e-1_f64 * t7390;
    let t7392 = t604 * t1017;
    let t7393 = t336 * t7392;
    let t7394 = t578 * t7393;
    (t7382, t7383, t7384, t7386, t7387, t7388, t7389, t7390, t7391, t7393, t7394)
}
