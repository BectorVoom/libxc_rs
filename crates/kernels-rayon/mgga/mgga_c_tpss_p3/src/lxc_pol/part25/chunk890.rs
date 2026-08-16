//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 890/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk890(t810: f64, t73: f64, t2157: f64, t806: f64, t2458: f64, t45: f64, t672: f64, t930: f64, t925: f64, t361: f64, t650: f64, t242: f64, t949: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8346 = t810 * t810;
    let t8347 = 1.0_f64 / t8346;
    let t8348 = t73 * t8347;
    let t8361 = t2157 * t806;
    let t8443 = t2458 * t45;
    let t8444 = 1.0_f64 / t8443;
    let t8455 = t672 * t930;
    let t8456 = t925 * t8455;
    let t8469 = t650 * t361;
    let t8471 = t242 * t8469 * t949;
    (t8346, t8347, t8348, t8361, t8444, t8456, t8469, t8471)
}
