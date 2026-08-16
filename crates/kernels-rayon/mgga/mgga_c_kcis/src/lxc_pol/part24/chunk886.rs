//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 886/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk886(t14497: f64, t18657: f64, t330: f64, t6539: f64, t829: f64, t2894: f64, t14518: f64, t14527: f64, t14529: f64, t19190: f64, t19194: f64, t19197: f64, t19200: f64, t19203: f64, t9883: f64, t991: f64, t9918: f64) -> f64 {
    let t19206 = t14497 * t18657;
    let t19209 = t6539 * t330;
    let t19210 = t19209 * t829;
    let t19211 = t2894 * t19210;
    let t19214 = t14518 - t14527 - t14529 / 648.0_f64 + t9883 - t9918 / 1296.0_f64 - t991 * t19190 / 144.0_f64 - t19194 / 432.0_f64 - t991 * t19197 / 216.0_f64 - t991 * t19200 / 36.0_f64 + 7.0_f64 / 648.0_f64 * t991 * t19203 + t991 * t19206 / 54.0_f64 - t991 * t19211 / 288.0_f64;
    t19214
}
