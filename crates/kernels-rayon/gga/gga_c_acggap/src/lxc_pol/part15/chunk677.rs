//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 677/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk677(t301: f64, t7381: f64, t7380: f64, t1983: f64, t372: f64, t2095: f64, t355: f64, t429: f64, t2016: f64, t2056: f64, t1981: f64, t576: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7382 = t7381 * t301;
    let t7383 = t7380 * t7382;
    let t7384 = t7383 / 32.0_f64;
    let t7386 = t1983 * t372;
    let t7387 = t2095 * t7386;
    let t7388 = t7387 / 96.0_f64;
    let t7389 = t429 * t355;
    let t7390 = t2095 * t7389;
    let t7391 = 0.1528125e-1_f64 * t7390;
    let t7396 = t2016 * t2056;
    let t7397 = 0.28015625e-1_f64 * t7396;
    let t7400 = t576 * t1981;
    (t7382, t7384, t7386, t7388, t7389, t7391, t7397, t7400)
}
