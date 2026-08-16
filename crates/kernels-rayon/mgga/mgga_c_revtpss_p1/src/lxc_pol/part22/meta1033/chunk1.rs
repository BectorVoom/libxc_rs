//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3617/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3617(t58207: f64, t68454: f64, t68529: f64, t68532: f64, t68535: f64, t68538: f64, t68540: f64, t68543: f64, t68546: f64, t68548: f64, t68550: f64, t68553: f64, t68556: f64, t68559: f64, t68561: f64) -> f64 {
    let t68564 = 0.44152e0_f64 * t68529 - 0.8585111111111111111e-1_f64 * t68532 + 0.33114e0_f64 * t68535 - 0.49057777777777777777e-1_f64 * t58207 - 0.44152e0_f64 * t68538 - 0.66228e0_f64 * t68540 + 0.16557e0_f64 * t68543 + 0.49671e0_f64 * t68546 + 0.73586666666666666667e-1_f64 * t68548 + 0.22076e0_f64 * t68550 - 0.5519e-1_f64 * t68553 + 0.36793333333333333333e-1_f64 * t68556 + 0.776775e1_f64 * t68559 - 0.16504875e0_f64 * t68561 - 0.80513333333333333333e0_f64 * t68454;
    t68564
}
