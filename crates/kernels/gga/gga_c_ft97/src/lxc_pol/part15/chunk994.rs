//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 994/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk994<F: Float>(t238: F, t88352: F, t88416: F, t88480: F, t88536: F, t88699: F, t88809: F, t88875: F, t88935: F, t192: F, t2506: F, t42218: F, t53123: F, t67103: F, t67288: F, t743: F, t80893: F, t80911: F, t80913: F, t80915: F, t80942: F, t80961: F, t81006: F, t81008: F, t88289: F, t88294: F, t92: F) -> (F, F) {
    let t239 = 0.1e-59 < t238;
    let t88939 = piecewise3(t239, t88352 + t88416 + t88480 + t88536 + t88699 + t88809 + t88875 + t88935, 0.0);
    let t88952 = -8.0 / 9.0 * t67103 + 8.0 / 3.0 * t80893 + 6.0 * t92 * t192 * t2506 * t88289 + 24.0 * t92 * t192 * t42218 * t88294 - t92 * t192 * t743 * t88939 + 8.0 / 9.0 * t80911 - 8.0 / 9.0 * t80913 + 8.0 / 3.0 * t80915 + 4.0 / 9.0 * t80942 - 8.0 * t80961 + 112.0 / 27.0 * t53123 + 16.0 / 9.0 * t67288 - 4.0 / 3.0 * t81006 - 4.0 / 3.0 * t81008;
    (t88939, t88952)
}
