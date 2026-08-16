//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1498/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1498(t31027: f64, t31440: f64, t31032: f64, t31444: f64, t108: f64, t1513: f64, t116912: f64, t31417: f64, t31421: f64, t101460: f64, t10199: f64, t117183: f64, t117184: f64, t117186: f64, t117188: f64, t117190: f64, t117198: f64, t117218: f64, t117226: f64, t117544: f64, t117545: f64, t1509: f64, t2194: f64, t2358: f64, t2362: f64, t2366: f64, t31035: f64, t31142: f64, t31149: f64, t31433: f64, t36308: f64, t36315: f64, t4279: f64, t8258: f64, t8267: f64, t8311: f64, t8315: f64) -> f64 {
    let t117976 = 20.0_f64 / 9.0_f64 * t31027 * t31440;
    let t117978 = 20.0_f64 / 27.0_f64 * t31032 * t31444;
    let t117997 = t108 * t1513;
    let t118009 = 4.0_f64 * t116912 * t31417;
    let t118011 = 20.0_f64 / 9.0_f64 * t31027 * t31421;
    let t118017 = -25.0_f64 / 18.0_f64 * t8258 * t31433 * t31142 - t117976 + t117978 + 5.0_f64 / 12.0_f64 * t8258 * t8315 * t1509 * t2366 - 5.0_f64 / 6.0_f64 * t117544 * t8315 * t117545 - 5.0_f64 / 36.0_f64 * t8267 * t31149 * t1509 * t2362 + 5.0_f64 / 24.0_f64 * t10199 * t2194 * t108 + 44.0_f64 / 9.0_f64 * t117184 - 110.0_f64 / 27.0_f64 * t117186 - 2.0_f64 / 3.0_f64 * t117188 + 5.0_f64 / 9.0_f64 * t117190 + t117183 - 5.0_f64 / 2.0_f64 * t36308 * t117997 * t31142 + 5.0_f64 / 9.0_f64 * t36315 * t4279 * t31142 + 5.0_f64 / 108.0_f64 * t8267 * t117218 * t1509 * t2358 + t118009 - t118011 - 3.0_f64 / 2.0_f64 * t31035 * t8311 * t101460 + 10.0_f64 / 27.0_f64 * t117198 + 2.0_f64 * t117226;
    t118017
}
