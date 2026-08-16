//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2266/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2266(t10416: f64, t7741: f64, t13435: f64, t2322: f64, t28042: f64, t13440: f64, t5523: f64, t101407: f64, t101517: f64, t101519: f64, t101521: f64, t101524: f64, t101526: f64, t101528: f64, t101530: f64, t97593: f64) -> f64 {
    let t101532 = 2.0_f64 * t10416 * t7741;
    let t101534 = 4.0_f64 * t13435 * t7741;
    let t101536 = 4.0_f64 * t2322 * t28042;
    let t101538 = 2.0_f64 * t13440 * t7741;
    let t101540 = 4.0_f64 * t5523 * t28042;
    let t101542 = t101517 + t101519 + t101521 + t101524 + t101526 + t101528 + t101530 + t101532 + t101534 + t101536 + t101538 + t101540 + 2.0_f64 * t97593 + t101407;
    t101542
}
