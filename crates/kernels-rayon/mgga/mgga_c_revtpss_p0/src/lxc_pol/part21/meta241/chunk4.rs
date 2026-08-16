//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1413/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1413(t33: f64, t3351: f64, t516: f64, t9350: f64, t9351: f64, t9354: f64, t9357: f64, t162: f64, t9348: f64, zeta_threshold: f64) -> f64 {
    let t34 = t33 <= zeta_threshold;
    let t9361 = piecewise3(t34, 0.0_f64, -8.0_f64 / 27.0_f64 * t9350 * t9351 + 4.0_f64 / 3.0_f64 * t9354 * t3351 + 4.0_f64 / 3.0_f64 * t516 * t9357);
    let t9363 = (t9348 + t9361) * t162;
    t9363
}
