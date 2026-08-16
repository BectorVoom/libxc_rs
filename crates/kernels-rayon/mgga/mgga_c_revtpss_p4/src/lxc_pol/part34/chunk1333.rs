//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1333/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1333(t30112: f64, t7898: f64, t29506: f64, t7935: f64, t114401: f64, t508: f64, t651: f64, t29583: f64, t1450: f64, t22809: f64, t2014: f64, t7237: f64) -> (f64, f64, f64, f64, f64) {
    let t114768 = 3.0_f64 * t7898 * t30112;
    let t114770 = 3.0_f64 * t29506 * t7935;
    let t114773 = 2.0_f64 * t651 * t508 * t114401;
    let t114775 = 18.0_f64 * t7898 * t29583;
    let t114776 = t1450 * t22809;
    let t114779 = 3.0_f64 * t2014 * t7237 * t114776;
    (t114768, t114770, t114773, t114775, t114779)
}
