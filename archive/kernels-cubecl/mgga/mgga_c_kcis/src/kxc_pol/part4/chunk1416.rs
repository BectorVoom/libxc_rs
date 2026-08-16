//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1416/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1416<F: Float>(t18403: F, t233: F, t1876: F, t2802: F, t12998: F, t13000: F, t18380: F, t18383: F, t18385: F, t18386: F, t18388: F, t18391: F, t18393: F, t18396: F, t18398: F, t9267: F, t9270: F, t9278: F, t9281: F) -> F {
    let t18404 = t233 * t18403;
    let t18406 = t2802 * t1876;
    let t18407 = t233 * t18406;
    let t18410 = -t18380 / F::cast_from(16.0_f64) - t18383 / F::cast_from(8.0_f64) + t18385 + t18386 / F::cast_from(8.0_f64) - t9278 + t9267 + t18388 / F::cast_from(8.0_f64) - t18391 / F::cast_from(8.0_f64) + t9281 + t18393 / F::cast_from(16.0_f64) - t18396 / F::cast_from(16.0_f64) + t12998 + t18398 / F::cast_from(16.0_f64) - t18404 / F::cast_from(16.0_f64) - t18407 / F::cast_from(16.0_f64) - t9270 + F::cast_from(2.0_f64) * t13000;
    t18410
}
