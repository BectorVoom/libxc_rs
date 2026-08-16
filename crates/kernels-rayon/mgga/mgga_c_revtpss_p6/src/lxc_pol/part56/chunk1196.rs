//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1196/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1196(t127346: f64, t127349: f64, t127357: f64, t127359: f64, t127361: f64, t127366: f64, t127369: f64, t127371: f64, t127373: f64, t127375: f64, t127378: f64, t127384: f64, t127385: f64, t129468: f64, t129471: f64, t129473: f64, t1453: f64, t29427: f64, t33346: f64, t34880: f64, t4293: f64, t7591: f64) -> f64 {
    let t132116 = t1453 * t34880 - 4.0_f64 * t29427 * t7591 - 2.0_f64 * t33346 * t4293 - t127346 + t127349 - t127357 - t127359 + t127361 - t127366 - t127369 - t127371 - t127373 - t127375 - t127378 - t127384 - t127385 - 4.0_f64 * t129468 - 4.0_f64 * t129471 - 4.0_f64 * t129473;
    t132116
}
