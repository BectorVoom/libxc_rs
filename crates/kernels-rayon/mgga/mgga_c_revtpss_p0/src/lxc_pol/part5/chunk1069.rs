//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1069/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1069(t4537: f64, t892: f64, t123: f64, t1534: f64, t2630: f64, t1469: f64, t749: f64, t606: f64, t4401: f64, t4391: f64, t705: f64, t2615: f64, t4311: f64) -> (f64, f64, f64, f64, f64) {
    let t14353 = t4537 * t892;
    let t14362 = t1534 * t123;
    let t14363 = t14362 * t2630;
    let t14369 = t749 * t1469;
    let t14370 = t14369 * t606;
    let t14372 = 24.0_f64 * t4401 * t14370;
    let t14386 = t705 * t4391;
    let t14433 = 8.0_f64 * t4311 * t2615;
    (t14353, t14363, t14372, t14386, t14433)
}
