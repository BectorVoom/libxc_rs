//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1201/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1201(t1967: f64, t816: f64, t1014: f64, t65: f64, t3252: f64, t3204: f64, t7131: f64, t1078: f64, t11239: f64, t1035: f64, t1983: f64, t4975: f64, t988: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27526 = t1967 * t816;
    let t27527 = t65 * t1014;
    let t27531 = t65 * t3252;
    let t27536 = t3204 * t7131;
    let t27638 = t11239 * t1078;
    let t27639 = t27638 * t1035;
    let t27640 = t1983 * t27639;
    let t27652 = t4975 * t988;
    (t27526, t27527, t27531, t27536, t27638, t27639, t27640, t27652)
}
