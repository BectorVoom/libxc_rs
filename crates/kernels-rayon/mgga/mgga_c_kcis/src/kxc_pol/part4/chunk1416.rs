//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1416/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1416(t18403: f64, t233: f64, t1876: f64, t2802: f64, t12998: f64, t13000: f64, t18380: f64, t18383: f64, t18385: f64, t18386: f64, t18388: f64, t18391: f64, t18393: f64, t18396: f64, t18398: f64, t9267: f64, t9270: f64, t9278: f64, t9281: f64) -> f64 {
    let t18404 = t233 * t18403;
    let t18406 = t2802 * t1876;
    let t18407 = t233 * t18406;
    let t18410 = -t18380 / 16.0_f64 - t18383 / 8.0_f64 + t18385 + t18386 / 8.0_f64 - t9278 + t9267 + t18388 / 8.0_f64 - t18391 / 8.0_f64 + t9281 + t18393 / 16.0_f64 - t18396 / 16.0_f64 + t12998 + t18398 / 16.0_f64 - t18404 / 16.0_f64 - t18407 / 16.0_f64 - t9270 + 2.0_f64 * t13000;
    t18410
}
