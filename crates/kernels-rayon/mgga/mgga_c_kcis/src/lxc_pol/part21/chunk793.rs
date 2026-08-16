//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 793/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk793(t209: f64, t698: f64, t9215: f64, t2399: f64, t2406: f64, t2412: f64, t4879: f64, t63: f64, t696: f64, t702: f64, t75: f64, t9195: f64, t9206: f64, t9211: f64) -> f64 {
    let t9217 = t209 * t698 * t9215;
    let t9220 = -455.0_f64 / 1296.0_f64 * t63 * t4879 * t75 - 35.0_f64 / 144.0_f64 * t9195 * t702 - 7.0_f64 / 48.0_f64 * t2399 * t2406 + 7.0_f64 / 96.0_f64 * t2399 * t2412 - t696 * t9206 / 16.0_f64 + t696 * t9211 / 16.0_f64 - t696 * t9217 / 96.0_f64;
    t9220
}
