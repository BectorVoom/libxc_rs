//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 918/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk918(t1150: f64, t6470: f64, t1131: f64, t3435: f64, t6438: f64, t3433: f64, t3439: f64, t5044: f64, t6423: f64, t6427: f64, t6431: f64, t1744: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6471 = t6470 * t1150;
    let t6473 = 1.0_f64 * t1131 * t6471;
    let t6474 = t6438 * t3435;
    let t6476 = 0.16081979498692535067e2_f64 * t3433 * t6474;
    let t6481 = t3439 - 0.11415555555555555555e-1_f64 * t5044 - 0.11415555555555555555e-1_f64 * t6423 + 0.34246666666666666666e-1_f64 * t6427 + 0.17123333333333333333e-1_f64 * t6431;
    let t6486 = t1744 * t1744;
    (t6471, t6473, t6474, t6476, t6481, t6486)
}
