//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1474/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1474(t1459: f64, t1461: f64, t1916: f64, t1918: f64, t2187: f64, t2189: f64, t31340: f64, t31359: f64, t31362: f64, t31365: f64, t31371: f64, t31374: f64, t572: f64, t573: f64, t5795: f64, t5802: f64, t5805: f64, t8289: f64, t8296: f64, t8299: f64, t8377: f64, t8383: f64, t8386: f64) -> f64 {
    let t31377 = 6.0_f64 * t1459 * t8383 + 3.0_f64 * t1459 * t8386 + 3.0_f64 * t1461 * t8377 + 6.0_f64 * t1916 * t8296 + 3.0_f64 * t1916 * t8299 + 3.0_f64 * t1918 * t8289 + 6.0_f64 * t2187 * t5802 + 3.0_f64 * t2187 * t5805 + 3.0_f64 * t2189 * t5795 + t31340 * t573 + 6.0_f64 * t31359 * t572 + 6.0_f64 * t31362 * t572 + 6.0_f64 * t31365 * t572 + 6.0_f64 * t31371 * t572 + 3.0_f64 * t31374 * t572;
    t31377
}
