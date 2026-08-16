//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 537/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk537(t1106: f64, t1181: f64, t423: f64, t3361: f64, t1111: f64, t1165: f64, t3189: f64, t160: f64, t413: f64, t168: f64, t1160: f64, t1167: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3363 = t1181 * t423 * t1106;
    let t3364 = t3361 * t3363;
    let t3367 = t1165 * t3189 * t1111;
    let t3368 = t3361 * t3367;
    let t3370 = t160 * t413;
    let t3371 = t3370 * t168;
    let t3372 = t1160 * t3371;
    let t3373 = t3372 * t1167;
    (t3363, t3364, t3367, t3368, t3370, t3371, t3372, t3373)
}
