//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2840/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2840(t23289: f64, t2741: f64, t2661: f64, t2662: f64, t6035: f64, t61625: f64, t124: f64, t40782: f64, t50943: f64, t50955: f64, t50978: f64, t62021: f64, t62029: f64, t62033: f64, t62045: f64, t62056: f64, t62058: f64, t62069: f64, t62072: f64, t62089: f64, t62095: f64, t62105: f64, t76421: f64, t799: f64, t800: f64) -> f64 {
    let t76793 = t2741 * t23289;
    let t76797 = t2661 * t2662 * t61625 * t6035;
    let t76800 = -0.54885603034028154483e-3_f64 * t50943 + 0.85748036236139473944e-4_f64 * t62021 + t50955 - 0.32524801797942610064e-2_f64 * t62029 - 0.30492001685571196935e-3_f64 * t62033 - 0.24009450146119052704e-1_f64 * t62045 - t799 * t800 * t124 * t76421 / 48.0_f64 + 7.0_f64 / 4.0_f64 * t62056 - 7.0_f64 / 8.0_f64 * t62058 + 0.76230004213927992336e-5_f64 * t62069 - 0.15246000842785598467e-4_f64 * t62072 + t50978 + 0.15117061203111996148e0_f64 * t40782 + 35.0_f64 / 24.0_f64 * t62089 - 35.0_f64 / 72.0_f64 * t62095 + 0.10003937560882938627e-2_f64 * t76793 - 0.85748036236139473942e-4_f64 * t76797 - 0.22869001264178397701e-3_f64 * t62105;
    t76800
}
