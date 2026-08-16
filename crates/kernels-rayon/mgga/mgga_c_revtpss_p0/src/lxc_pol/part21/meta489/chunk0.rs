//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2073/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2073(t15191: f64, t15197: f64, t11134: f64, t11136: f64, t11138: f64, t11140: f64, t11334: f64, t11338: f64, t11339: f64, t11366: f64, t11368: f64, t15221: f64, t15230: f64) -> (f64, f64, f64) {
    let t15457 = 0.19931111111111111111e0_f64 * t15191;
    let t15459 = 0.10954222222222222222e0_f64 * t15197;
    let t15472 = -t11334 - t11338 + 0.3071625e0_f64 * t15221 + 0.18257037037037037037e-1_f64 * t11339 - 0.19931111111111111111e0_f64 * t11138 - 0.26574814814814814816e0_f64 * t11134 + 0.99655555555555555557e-1_f64 * t11140 + 0.66437037037037037038e-1_f64 * t11136 - 0.18257037037037037037e0_f64 * t11366 + 0.54771111111111111111e-1_f64 * t11368 + 0.1898925e1_f64 * t15230;
    (t15457, t15459, t15472)
}
