//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1135/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1135(t238: f64, t88352: f64, t88416: f64, t88480: f64, t88536: f64, t88699: f64, t88809: f64, t88875: f64, t88935: f64, t192: f64, t2506: f64, t42218: f64, t53123: f64, t67103: f64, t67288: f64, t743: f64, t80893: f64, t80911: f64, t80913: f64, t80915: f64, t80942: f64, t80961: f64, t81006: f64, t81008: f64, t88289: f64, t88294: f64, t92: f64) -> (f64, f64) {
    let t239 = 0.1e-59_f64 < t238;
    let t88939 = piecewise3(t239, t88352 + t88416 + t88480 + t88536 + t88699 + t88809 + t88875 + t88935, 0.0_f64);
    let t88952 = -8.0_f64 / 9.0_f64 * t67103 + 8.0_f64 / 3.0_f64 * t80893 + 6.0_f64 * t92 * t192 * t2506 * t88289 + 24.0_f64 * t92 * t192 * t42218 * t88294 - t92 * t192 * t743 * t88939 + 8.0_f64 / 9.0_f64 * t80911 - 8.0_f64 / 9.0_f64 * t80913 + 8.0_f64 / 3.0_f64 * t80915 + 4.0_f64 / 9.0_f64 * t80942 - 8.0_f64 * t80961 + 112.0_f64 / 27.0_f64 * t53123 + 16.0_f64 / 9.0_f64 * t67288 - 4.0_f64 / 3.0_f64 * t81006 - 4.0_f64 / 3.0_f64 * t81008;
    (t88939, t88952)
}
