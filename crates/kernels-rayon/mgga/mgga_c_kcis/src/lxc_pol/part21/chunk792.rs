//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 792/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk792(t60: f64, t9060: f64, t9184: f64, t20: f64, t2394: f64, t63: f64, t697: f64, t2404: f64, t700: f64, t209: f64, t2403: f64, t2410: f64, t8747: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t70 = 0.0_f64 < t60;
    let t9185 = t9060 + t9184;
    let t9194 = t2394 * t20;
    let t9195 = t63 * t9194;
    let t9202 = t697 * t697;
    let t9203 = 1.0_f64 / t9202;
    let t9204 = t2404 * t700;
    let t9206 = t209 * t9203 * t9204;
    let t9209 = t2403 * t700;
    let t9211 = t209 * t9209 * t2410;
    let t9215 = piecewise3(t70, t8747, -t8747);
    (t9185, t9194, t9195, t9203, t9204, t9206, t9211, t9215)
}
