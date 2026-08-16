//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3290/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3290(t2475: f64, t5962: f64, t10696: f64, t5966: f64, t14468: f64, t14643: f64, t14649: f64, t14653: f64, t14656: f64, t18392: f64, t18586: f64, t18592: f64, t18599: f64, t18600: f64, t18603: f64, t18608: f64, t18609: f64, t2394: f64, t2430: f64, t4415: f64, t4416: f64, t775: f64, t833: f64, t853: f64) -> (f64, f64, f64) {
    let t62351 = t2475 * t5962;
    let t62361 = t10696 * t5966;
    let t62383 = -24.0_f64 * t18392 * t4415 * t775 * t853 - 24.0_f64 * t14468 * t4415 * t4416 + 60.0_f64 * t18599 * t2430 * t4415 - 12.0_f64 * t18608 * t2430 * t4415 + 60.0_f64 * t2394 * t4415 * t62351 - 360.0_f64 * t2394 * t4415 * t62361 + 120.0_f64 * t14643 * t18600 - 48.0_f64 * t14643 * t18603 - 24.0_f64 * t14643 * t18609 + 120.0_f64 * t14649 * t18592 - 48.0_f64 * t14653 * t18592 - 24.0_f64 * t14656 * t18592 + 6.0_f64 * t18586 * t833;
    (t62351, t62361, t62383)
}
