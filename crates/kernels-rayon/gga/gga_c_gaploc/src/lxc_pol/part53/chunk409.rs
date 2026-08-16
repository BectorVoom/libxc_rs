//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 409/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk409(t3437: f64, t943: f64, t2936: f64, t948: f64, t2508: f64, t2949: f64, t883: f64, t2562: f64, t2958: f64, t935: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3438 = t943 * t3437;
    let t3439 = 0.32043859292259267849e-3_f64 * t3438;
    let t3440 = t2936 * t948;
    let t3442 = 0.23071578690426672851e-1_f64 * t2508 * t3440;
    let t3443 = t883 * t2949;
    let t3444 = t2562 * t3443;
    let t3445 = t943 * t3444;
    let t3446 = 0.32043859292259267849e-3_f64 * t3445;
    let t3447 = t2958 * t935;
    (t3439, t3440, t3442, t3444, t3446, t3447)
}
