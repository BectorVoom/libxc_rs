//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1230/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1230(t129467: f64, t1937: f64, t2126: f64, t4292: f64, t34446: f64, t6993: f64, t127336: f64, t127340: f64, t127341: f64, t127346: f64, t129455: f64, t129457: f64, t129459: f64, t129461: f64, t129463: f64, t129465: f64) -> (f64, f64) {
    let t129468 = t129467 * t1937;
    let t129470 = t2126 * t4292;
    let t129471 = t129470 * t1937;
    let t129473 = t34446 * t6993;
    let t129476 = -t129455 - 3.0_f64 * t127336 - t127340 - 2.0_f64 * t129457 - 2.0_f64 * t129459 - 2.0_f64 * t129461 - 2.0_f64 * t129463 - 2.0_f64 * t129465 - 2.0_f64 * t129468 - 2.0_f64 * t129471 - 2.0_f64 * t129473 + 3.0_f64 * t127341 - t127346;
    (t129470, t129476)
}
