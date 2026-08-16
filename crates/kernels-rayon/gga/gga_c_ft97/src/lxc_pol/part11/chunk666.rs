//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 666/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk666(t9204: f64, t9242: f64, t579: f64, t91: f64, t153: f64, t525: f64, t631: f64, t637: f64, t7242: f64, t2087: f64, t590: f64, t9014: f64, t9024: f64, t9028: f64, t9032: f64, t9057: f64, t9059: f64, t9062: f64, t9076: f64, t9080: f64, t9166: f64, t9170: f64) -> (f64, f64, f64, f64, f64) {
    let t9243 = t9204 + t9242;
    let t9245 = t91 * t579 * t9243;
    let t9252 = 1.0_f64 / t153 / t631 / t637 / t525 / t7242 / 4.0_f64;
    let t9253 = t2087 * t590;
    let t9255 = t91 * t9252 * t9253;
    let t9257 = -2.0_f64 / 3.0_f64 * t9059 - 2.0_f64 * t9076 - 2.0_f64 * t9080 - t9166 - t9014 / 3.0_f64 - 3.0_f64 / 4.0_f64 * t9170 + 6.0_f64 * t9024 - 10.0_f64 / 27.0_f64 * t9028 - 2.0_f64 * t9032 + 4.0_f64 / 3.0_f64 * t9057 - 2.0_f64 / 3.0_f64 * t9062 + t9245 / 2.0_f64 + 3.0_f64 / 8.0_f64 * t9255;
    (t9243, t9245, t9252, t9255, t9257)
}
