//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1151/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1151(t127354: f64, t28196: f64, t28198: f64, t28056: f64, t8634: f64, t32129: f64, t7898: f64, t13426: f64, t8461: f64, t18227: f64, t32110: f64, t4248: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t127357 = 6.0_f64 * t28196 * t127354 * t28198;
    let t127359 = 4.0_f64 * t8634 * t28056;
    let t127361 = 2.0_f64 * t7898 * t32129;
    let t127365 = t13426 * t8461;
    let t127366 = 2.0_f64 * t127365;
    let t127368 = t18227 * t8461;
    let t127369 = 2.0_f64 * t127368;
    let t127370 = t4248 * t32110;
    (t127357, t127359, t127361, t127366, t127369, t127370)
}
