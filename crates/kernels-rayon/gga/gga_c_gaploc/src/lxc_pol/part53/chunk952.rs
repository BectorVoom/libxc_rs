//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 952/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk952(t11977: f64, t2268: f64, t2349: f64, t1063: f64, t13829: f64, t448: f64, t13732: f64, t6313: f64, t105: f64, t169: f64, t172: f64, t452: f64, t46952: f64) -> (f64, f64, f64, f64) {
    let t46970 = t2268 * t11977 * t2349;
    let t46979 = 0.28455006635676149599e-1_f64 * t1063 * t13829 * t448;
    let t46980 = t6313 * t13732;
    let t46991 = 0.28455006635676149599e-1_f64 * t105 * t452 * t46952 * t169 * t172;
    (t46970, t46979, t46980, t46991)
}
