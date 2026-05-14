//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1235/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1235<F: Float>(t169: F, t18390: F, t446: F, t1881: F, t4505: F, t2132: F, t3708: F, t3709: F, t18376: F, t234: F, t441: F, t233: F, t1876: F, t2802: F, t12998: F, t13000: F, t18380: F, t18383: F, t18385: F, t18386: F, t18388: F, t9267: F, t9270: F, t9278: F, t9281: F, zeta_threshold: F) -> (F,) {
    let t170 = t169 <= zeta_threshold;
    let t18391 = t446 * t18390;
    let t18393 = t1881 * t4505;
    let t18395 = t3708 * t2132;
    let t18396 = t446 * t18395;
    let t18398 = t1881 * t3709;
    let t18401 = piecewise3(t170, 0.0, -t18376);
    let t18402 = t234 * t18401;
    let t18403 = t18402 * t441;
    let t18404 = t233 * t18403;
    let t18406 = t2802 * t1876;
    let t18407 = t233 * t18406;
    let t18410 = -t18380 / 16.0 - t18383 / 8.0 + t18385 + t18386 / 8.0 - t9278 + t9267 + t18388 / 8.0 - t18391 / 8.0 + t9281 + t18393 / 16.0 - t18396 / 16.0 + t12998 + t18398 / 16.0 - t18404 / 16.0 - t18407 / 16.0 - t9270 + 2.0 * t13000;
    (t18410,)
}
