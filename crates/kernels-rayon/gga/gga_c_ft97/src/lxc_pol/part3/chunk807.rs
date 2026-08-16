//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 807/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk807(t16023: f64, t16081: f64, t16144: f64, t16194: f64, t16227: f64, t16284: f64, t16332: f64, t16544: f64, t108: f64, t15594: f64, t15596: f64, t15599: f64, t15968: f64, t2976: f64, t3109: f64, t3289: f64, t438: f64, t4415: f64, t4501: f64, t4621: f64, t497: f64, t88: f64, t948: f64, t984: f64) -> f64 {
    let t16547 = t16023 + t16081 + t16144 + t16194 + t16227 + t16284 + t16332 + t16544;
    let t16549 = -t108 * t15594 - t108 * t15596 - t108 * t15599 - t108 * t15968 - t16547 * t88 - 2.0_f64 * t2976 * t984 - 2.0_f64 * t3109 * t984 - 2.0_f64 * t3289 * t948 - t438 * t4621 - t4415 * t497 - t4501 * t497;
    t16549
}
