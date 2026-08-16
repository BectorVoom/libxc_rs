//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 927/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk927(t10336: f64, t221: f64, t339: f64, t1032: f64, t3082: f64, t2393: f64, t374: f64, t376: f64, t370: f64, t3158: f64, t964: f64, t10335: f64) -> (f64, f64, f64, f64, f64) {
    let t10337 = t221 * t10336;
    let t10339 = 0.3086419753086419753e-3_f64 * t339 * t10337;
    let t10372 = t1032 * t3082;
    let t10375 = t374 * t2393 * t376;
    let t10377 = t370 * t10375 / 10368.0_f64;
    let t10381 = t964 * t3158;
    let t10383 = t221 * t10335;
    (t10339, t10372, t10377, t10381, t10383)
}
