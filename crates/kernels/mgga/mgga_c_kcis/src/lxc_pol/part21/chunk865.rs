//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 865/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk865<F: Float>(t14181: F, t14246: F, t14311: F, t14372: F, t1022: F, t1096: F, t1092: F, t3201: F, t3204: F, t4580: F, t3200: F, t4566: F, t4554: F, t1714: F, t9562: F, t20: F, t284: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14374 = t14181 + t14246 + t14311 + t14372;
    let t14375 = t1022 * t14374;
    let t14376 = t1096 * t14375;
    let t14377 = t1092 * t14376;
    let t14381 = t3201 * t1022;
    let t14382 = t4580 * t3204;
    let t14383 = t14381 * t14382;
    let t14384 = t3200 * t14383;
    let t14386 = t4566 * t3204;
    let t14387 = t14381 * t14386;
    let t14388 = t4554 * t14387;
    let t14390 = t9562 * t1714;
    let t14393 = t284 * t20;
    (t14374, t14375, t14377, t14382, t14384, t14386, t14388, t14390, t14393)
}
