//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 972/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk972(t23628: f64, t24185: f64, t1102: f64, t11108: f64, t198: f64, t23562: f64, t23564: f64, t23567: f64, t23570: f64, t23571: f64, t23651: f64, t23665: f64, t23698: f64, t23769: f64, t23772: f64, t23816: f64, t23818: f64, t336: f64) -> f64 {
    let t24186 = t23628 + t24185;
    let t24190 = t1102 * t198 * t24186 * t336 + 2.0_f64 * t11108 * t198 * t23571 * t336 + t23562 - t23564 + t23567 - t23570 - t23651 - t23665 - t23698 - t23769 + t23772 + t23816 + t23818;
    t24190
}
