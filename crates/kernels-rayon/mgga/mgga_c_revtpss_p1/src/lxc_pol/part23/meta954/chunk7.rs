//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3180/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3180(t1222: f64, t17240: f64, t24244: f64, t20982: f64, t20986: f64, t21126: f64, t21129: f64, t21239: f64, t5312: f64, t5373: f64, t5391: f64, t57480: f64, t57491: f64, t70733: f64, t81173: f64, t81182: f64, t81212: f64) -> f64 {
    let t83504 = t1222 * t17240 * t24244;
    let t83526 = t57491 - t83504 / 144.0_f64 + 0.85748036236139473944e-3_f64 * t70733 + 7.0_f64 / 81.0_f64 * t5373 * t21129 + 35.0_f64 / 972.0_f64 * t1222 * t57480 * t81212 - t5373 * t21126 / 27.0_f64 - 2.0_f64 / 9.0_f64 * t5373 * t21239 + t1222 * t5312 * t81182 / 216.0_f64 + t1222 * t5312 * t81173 / 6.0_f64 + 0.91464571985215438872e-2_f64 * t5391 * t20982 + 0.13719685797782315831e-1_f64 * t5391 * t20986;
    t83526
}
