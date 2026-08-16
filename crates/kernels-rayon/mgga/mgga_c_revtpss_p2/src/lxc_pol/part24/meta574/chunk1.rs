//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1758/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1758(t1132: f64, t90450: f64, t3407: f64, t90419: f64, t141: f64, t3417: f64, t89841: f64, t89826: f64, t81230: f64, t81232: f64, t81234: f64, t81425: f64, t81427: f64, t81429: f64, t89828: f64, t89843: f64, t89847: f64, t89855: f64) -> (f64, f64, f64, f64, f64) {
    let t90459 = t1132 * t90450;
    let t90464 = t3407 * t90419;
    let t90470 = t141 * t3417 * t89841;
    let t90473 = t141 * t3417 * t89826;
    let t90478 = -0.72462e1_f64 * t89828 + 0.258925e1_f64 * t90459 - 0.22076e0_f64 * t81425 + 0.44152e0_f64 * t81427 - 0.132456e1_f64 * t81429 + 0.247573125e0_f64 * t90464 - 0.80513333333333333332e0_f64 * t89843 + 0.108693e2_f64 * t89847 + 0.24154e1_f64 * t89855 - 0.11038e0_f64 * t90470 - 0.99342e0_f64 * t90473 - 0.44729629629629629629e0_f64 * t81230 + 0.16102666666666666667e1_f64 * t81232 - 0.24154e1_f64 * t81234;
    (t90459, t90464, t90470, t90473, t90478)
}
