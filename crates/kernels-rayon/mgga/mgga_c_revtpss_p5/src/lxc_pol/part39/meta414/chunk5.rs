//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1497/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1497(t116913: f64, t116915: f64, t116917: f64, t116927: f64, t116930: f64, t116932: f64, t116934: f64, t116936: f64, t116968: f64, t116969: f64, t116971: f64, t116995: f64) -> f64 {
    let t117572 = 2.0_f64 * t116913 + 20.0_f64 / 9.0_f64 * t116915 + 10.0_f64 / 27.0_f64 * t116917 + 44.0_f64 / 9.0_f64 * t116927 - 110.0_f64 / 27.0_f64 * t116930 - 2.0_f64 / 3.0_f64 * t116932 - 50.0_f64 / 27.0_f64 * t116934 + 5.0_f64 / 9.0_f64 * t116936 + t116968 + 110.0_f64 / 27.0_f64 * t116969 + 40.0_f64 / 27.0_f64 * t116971 - 20.0_f64 / 9.0_f64 * t116995;
    t117572
}
