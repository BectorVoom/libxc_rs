//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1160/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1160(t14363: f64, t775: f64, t890: f64, t1469: f64, t749: f64, t606: f64, t4401: f64, t10561: f64, t10563: f64, t2394: f64, t262: f64, t10569: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14364 = 0.10843581300301739842e-1_f64 * t14363;
    let t14365 = t890 * t775;
    let t14369 = t749 * t1469;
    let t14370 = t14369 * t606;
    let t14372 = 24.0_f64 * t4401 * t14370;
    let t14373 = 8.0_f64 * t10561;
    let t14374 = 2.0_f64 * t10563;
    let t14375 = t2394 * t262;
    let t14379 = 0.4883052614935078681e-3_f64 * t10569;
    (t14364, t14365, t14372, t14373, t14374, t14375, t14379)
}
