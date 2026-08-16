//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 972/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk972(t16126: f64, t16226: f64, t16244: f64, t16251: f64, t16253: f64, t16256: f64, t16259: f64, t16262: f64, t16266: f64, t16269: f64, t16273: f64, t16276: f64, t17915: f64, t601: f64) -> f64 {
    let t17919 = -t16126 - t16226 + t16251 - t16253 + t16256 + t16259 + t16262 - t16266 - t16269 - t16273 - t16276 - 0.3109e-1_f64 * t17915 * t601 - 0.19751789702565206229e-1_f64 * t16244;
    t17919
}
