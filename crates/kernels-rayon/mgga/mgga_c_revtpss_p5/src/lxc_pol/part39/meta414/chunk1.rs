//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1493/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1493(t31032: f64, t31280: f64, t46089: f64, t655: f64, t31288: f64, t116926: f64, t8355: f64, t31027: f64, t31264: f64, t116938: f64, t116957: f64, t117450: f64, t117457: f64, t13509: f64, t1504: f64, t1513: f64, t2: f64, t31039: f64, t31054: f64, t31287: f64, t4287: f64, t8258: f64, t8259: f64, t8267: f64) -> f64 {
    let t117460 = 50.0_f64 / 27.0_f64 * t31032 * t31280;
    let t117461 = t46089 * t655;
    let t117462 = t117461 * t31288;
    let t117470 = t116926 * t8355;
    let t117473 = 20.0_f64 / 9.0_f64 * t31027 * t31264;
    let t117477 = -t117450 - 5.0_f64 / 6.0_f64 * t8258 * t31039 * t4287 + t8258 * t8259 * t13509 / 4.0_f64 - 55.0_f64 / 27.0_f64 * t117457 - t117460 + 125.0_f64 / 72.0_f64 * t117462 - 25.0_f64 / 27.0_f64 * t8267 * t116957 * t1504 + 25.0_f64 / 36.0_f64 * t31287 * t31054 * t2 + 22.0_f64 / 9.0_f64 * t117470 + t117473 + 10.0_f64 / 9.0_f64 * t8258 * t116938 * t1513;
    t117477
}
