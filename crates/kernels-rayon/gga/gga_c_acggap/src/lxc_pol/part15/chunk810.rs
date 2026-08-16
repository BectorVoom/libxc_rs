//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 810/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk810(t8945: f64, t7772: f64, t7774: f64, t7790: f64, t7798: f64, t8268: f64, t8269: f64, t8271: f64, t8275: f64, t8276: f64, t8278: f64, t8943: f64, t8949: f64, t8953: f64, t8957: f64) -> f64 {
    let t9348 = 7.0_f64 / 144.0_f64 * t8945;
    let t9352 = -t7772 - t7774 - t8268 + t8269 - t8271 + t7790 + t7798 + t8275 - t8276 - t8278 + t8943 / 48.0_f64 - t9348 + 0.42874018118069736972e-3_f64 * t8949 - 0.31448092289604152069e-3_f64 * t8953 - 0.15724046144802076034e-2_f64 * t8957;
    t9352
}
