//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1056/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1056<F: Float>(t15466: F, t15481: F, t898: F, t338: F, t353: F, t1205: F, t3703: F, t2376: F, t2409: F, t4207: F, t8589: F, t1115: F, t14295: F, t14302: F, t14611: F, t14655: F, t14911: F, t14964: F, t15192: F, t15198: F, t15201: F, t15205: F, t15216: F, t15279: F, t15445: F, t2408: F, t3066: F, t3207: F, t335: F, t3921: F, t4083: F) -> (F, F, F, F, F, F, F) {
    let t15482 = t15466 + t15481;
    let t15483 = t898 * t15482;
    let t15485 = t338 * t353 * t15483;
    let t15490 = t1205 * t3703;
    let t15492 = t2409 * t2376 * t15490;
    let t15500 = t2409 * t8589 * t4207;
    let t15503 = -t3921 * t4083 / 96.0 - t1115 * t14911 / 48.0 - t15192 / 96.0 + t3066 * t15445 / 24.0 + 7.0 / 1152.0 * t14611 + t15198 / 12.0 - t335 * t15485 / 96.0 + t15201 / 384.0 - t15205 / 384.0 - t3207 * t15492 / 16.0 + t15216 / 24.0 + t14295 + 7.0 / 288.0 * t14655 - t14302 + t15279 / 768.0 - 7.0 / 72.0 * t14964 + t2408 * t15500 / 24.0;
    (t15482, t15483, t15485, t15490, t15492, t15500, t15503)
}
