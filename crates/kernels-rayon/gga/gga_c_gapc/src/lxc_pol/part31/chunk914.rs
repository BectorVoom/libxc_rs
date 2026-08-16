//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 914/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk914(t190: f64, t2207: f64, t10346: f64, t442: f64, t875: f64, t3439: f64, t6939: f64, t19: f64, t786: f64, t147: f64, t3296: f64, t2405: f64, t3188: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10347 = t2207 * t190;
    let t10348 = t10346 * t10347;
    let t10349 = t442 * t875;
    let t10350 = t3439 * t10349;
    let t10351 = t10348 * t10350;
    let t10353 = t6939 * t190;
    let t10354 = t10346 * t10353;
    let t10355 = t786 * t19;
    let t10356 = t10355 * t147;
    let t10357 = t3296 * t10356;
    let t10358 = t10354 * t10357;
    let t10360 = t3188 * t2405;
    (t10349, t10350, t10351, t10357, t10358, t10360)
}
