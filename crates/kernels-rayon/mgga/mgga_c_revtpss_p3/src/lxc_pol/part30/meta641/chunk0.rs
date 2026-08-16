//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2228/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2228(t17608: f64, t7617: f64, t17217: f64, t26880: f64, t17376: f64, t26843: f64, t26848: f64, t29010: f64, t3704: f64, t17720: f64, t7624: f64, t1252: f64, t17199: f64, t17204: f64, t17232: f64, t17589: f64, t3606: f64, t3613: f64, t97125: f64) -> f64 {
    let t104677 = t17608 * t7617;
    let t104680 = t26880 * t17217;
    let t104682 = t17376 * t26843;
    let t104685 = t17376 * t26848;
    let t104689 = 0.57165357490759649296e-3_f64 * t29010 * t3704;
    let t104691 = 0.6351706387862183255e-3_f64 * t7624 * t17720;
    let t104692 = 0.57165357490759649296e-3_f64 * t97125 + 0.57165357490759649296e-3_f64 * t26880 * t17589 - 0.11433071498151929859e-2_f64 * t7624 * t17232 - 0.57165357490759649296e-3_f64 * t7624 * t17199 - 0.17149607247227894789e-2_f64 * t7624 * t17204 + 0.85748036236139473944e-3_f64 * t104677 * t1252 + 0.3811023832717309953e-3_f64 * t104680 + 0.85748036236139473944e-3_f64 * t104682 * t3606 - 0.42874018118069736972e-3_f64 * t104685 * t3613 + t104689 + t104691;
    t104692
}
