//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3170/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3170(t1012: f64, t1222: f64, t1225: f64, t17649: f64, t17654: f64, t20767: f64, t20938: f64, t21111: f64, t21119: f64, t21210: f64, t5373: f64, t5381: f64, t57094: f64, t70278: f64, t70281: f64, t70300: f64, t70306: f64, t70990: f64, t71440: f64, t76397: f64, t83033: f64) -> f64 {
    let t83281 = -0.19055119163586549765e-3_f64 * t70278 - 0.1270341277572436651e-2_f64 * t70281 - 0.19055119163586549765e-2_f64 * t5381 * t21111 - 0.85748036236139473944e-3_f64 * t70300 + t5373 * t21210 / 36.0_f64 - t1222 * t1012 * t1225 * t76397 / 288.0_f64 + 0.42874018118069736972e-3_f64 * t70306 + 0.95275595817932748827e-4_f64 * t57094 + 0.91464571985215438872e-2_f64 * t70990 * t20767 + 0.91464571985215438872e-2_f64 * t71440 * t20938 - 0.85748036236139473944e-3_f64 * t17654 * t17649 * t83033 * t21119;
    t83281
}
