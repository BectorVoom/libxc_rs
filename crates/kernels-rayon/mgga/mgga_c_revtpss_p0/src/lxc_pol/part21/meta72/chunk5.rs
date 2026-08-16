//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 535/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk535(t1459: f64, t1461: f64, t572: f64, t573: f64, t578: f64, t582: f64, t586: f64, t590: f64, t594: f64, t598: f64, t4: f64, t604: f64) -> (f64, f64, f64) {
    let t1464 = t1459 * t573 + 3.0_f64 * t1461 * t572;
    let t1466 = -t578 - t582 - t586 - t590 - t594 - t598;
    let t1468 = -t4 - t604;
    (t1464, t1466, t1468)
}
