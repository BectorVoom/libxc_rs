//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1287/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1287(t50237: f64, t50247: f64, t50253: f64, t50275: f64, t50279: f64, t50281: f64, t50290: f64, t50291: f64, t50299: f64, t50309: f64, t50310: f64, t21640: f64, t50311: f64, t50327: f64, t50329: f64, t50335: f64, t50349: f64, t50353: f64, t50362: f64, t50363: f64, t50368: f64, t50371: f64) -> (f64, f64) {
    let t50589 = t50237 - t50247 + t50253 + t50275 - t50279 + t50281 + t50290 + t50291 + t50299 - t50309 + t50310;
    let t50590 = t50311 - t50327 - t50329 - t50335 + t21640 + t50349 + t50353 + t50362 - t50363 + t50368 + t50371;
    (t50589, t50590)
}
