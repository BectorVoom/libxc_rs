//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 888/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk888(t30: f64, t2257: f64, t513: f64, t9335: f64, t9336: f64, t9339: f64, t9344: f64, t33: f64, t527: f64, t1113: f64, t3842: f64, t3841: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t9348 = piecewise3(t31, 0.0_f64, -8.0_f64 / 27.0_f64 * t9335 * t9336 + 4.0_f64 / 3.0_f64 * t9339 * t2257 + 4.0_f64 / 3.0_f64 * t513 * t9344);
    let t9350 = 1.0_f64 / t527 / t33;
    let t9351 = t3842 * t1113;
    let t9354 = t3841 * t1113;
    let t9357 = -t9344;
    (t9348, t9350, t9351, t9354, t9357)
}
