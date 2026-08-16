//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1483/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1483(t116929: f64, t8358: f64, t31032: f64, t31280: f64, t46089: f64, t655: f64, t31288: f64, t116926: f64, t8355: f64, t31027: f64, t31264: f64, t31277: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t117457 = t116929 * t8358;
    let t117460 = 50.0_f64 / 27.0_f64 * t31032 * t31280;
    let t117461 = t46089 * t655;
    let t117462 = t117461 * t31288;
    let t117470 = t116926 * t8355;
    let t117473 = 20.0_f64 / 9.0_f64 * t31027 * t31264;
    let t117482 = 20.0_f64 / 9.0_f64 * t31027 * t31277;
    (t117457, t117460, t117462, t117470, t117473, t117482)
}
