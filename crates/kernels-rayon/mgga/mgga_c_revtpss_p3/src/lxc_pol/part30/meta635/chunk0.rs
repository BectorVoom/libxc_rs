//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2203/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2203(t13435: f64, t7741: f64, t2322: f64, t28042: f64, t13440: f64, t5523: f64, t25191: f64, t7898: f64, t1937: f64, t49686: f64, t75667: f64, t13426: f64, t6993: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t101534 = 4.0_f64 * t13435 * t7741;
    let t101536 = 4.0_f64 * t2322 * t28042;
    let t101538 = 2.0_f64 * t13440 * t7741;
    let t101540 = 4.0_f64 * t5523 * t28042;
    let t101546 = 6.0_f64 * t7898 * t25191;
    let t101548 = 2.0_f64 * t49686 * t1937;
    let t101550 = 4.0_f64 * t75667 * t1937;
    let t101552 = 4.0_f64 * t13426 * t6993;
    (t101534, t101536, t101538, t101540, t101546, t101548, t101550, t101552)
}
