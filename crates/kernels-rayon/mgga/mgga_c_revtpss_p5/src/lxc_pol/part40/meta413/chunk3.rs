//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1497/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1497(t2289: f64, t8399: f64, t31027: f64, t31424: f64, t101457: f64, t101463: f64, t116919: f64, t117228: f64, t117918: f64, t117920: f64, t117927: f64, t117932: f64, t117936: f64, t117938: f64, t13509: f64, t1509: f64, t1513: f64, t2: f64, t2340: f64, t2358: f64, t2362: f64, t2366: f64, t31035: f64, t31149: f64, t31287: f64, t31429: f64, t31433: f64, t4287: f64, t661: f64, t8258: f64, t8267: f64, t8311: f64, t8315: f64) -> f64 {
    let t117940 = t2289 * t8399;
    let t117943 = 4.0_f64 / 3.0_f64 * t31027 * t31424;
    let t117971 = t8258 * t8311 * t13509 / 4.0_f64 + t117918 - t117920 - 5.0_f64 / 12.0_f64 * t8258 * t31429 * t2366 + 25.0_f64 / 72.0_f64 * t8267 * t31433 * t2362 - 125.0_f64 / 72.0_f64 * t117927 + 5.0_f64 / 4.0_f64 * t31035 * t31429 * t2340 + 25.0_f64 / 108.0_f64 * t8267 * t117932 * t2358 - 55.0_f64 / 27.0_f64 * t117936 + 22.0_f64 / 9.0_f64 * t117938 + 55.0_f64 / 27.0_f64 * t117940 - t117943 - 20.0_f64 / 9.0_f64 * t117228 + 3.0_f64 * t116919 * t8311 * t101457 + 5.0_f64 / 18.0_f64 * t8258 * t31149 * t1513 * t2358 - 5.0_f64 / 4.0_f64 * t31035 * t8315 * t1509 * t2340 + 5.0_f64 / 18.0_f64 * t31287 * t31149 * t2 * t661 - 3.0_f64 / 4.0_f64 * t31035 * t8311 * t101463 + 5.0_f64 / 6.0_f64 * t8258 * t8315 * t4287 * t661 + 5.0_f64 / 12.0_f64 * t8258 * t8315 * t1513 * t2362;
    t117971
}
