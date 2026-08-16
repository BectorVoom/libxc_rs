//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2219/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2219(t670: f64, t7583: f64, t101530: f64, t101532: f64, t101534: f64, t101536: f64, t101538: f64, t101540: f64, t104115: f64, t104138: f64, t13514: f64, t1518: f64, t2371: f64, t27060: f64, t29427: f64, t29432: f64, t4292: f64, t7586: f64, t96706: f64) -> (f64, f64) {
    let t104416 = t7583 * t670;
    let t104427 = 4.0_f64 * t104115 * t670 + 2.0_f64 * t104138 * t1518 + 4.0_f64 * t104416 * t1518 + 2.0_f64 * t13514 * t7586 + 2.0_f64 * t1518 * t96706 + 2.0_f64 * t2371 * t29427 + 4.0_f64 * t27060 * t4292 + 4.0_f64 * t29432 * t4292 + t101530 + t101532 + t101534 + t101536 + t101538 + t101540;
    (t104416, t104427)
}
