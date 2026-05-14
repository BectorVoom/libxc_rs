//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1179/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1179<F: Float>(t11849: F, t14031: F, t11798: F, t12009: F, t14046: F, t15248: F, t11990: F, t338: F, t54244: F, t14024: F, t3805: F, t11644: F, t4028: F, t12080: F, t14101: F, t54355: F, t54378: F, t55607: F, t55609: F, t55623: F) -> (F,) {
    let t57195 = t14031 * t11849;
    let t57197 = t14031 * t11798;
    let t57199 = t14031 * t12009;
    let t57201 = t14046 * t15248;
    let t57204 = t54244 * t338 * t11990;
    let t57206 = t3805 * t14024;
    let t57208 = t4028 * t11644;
    let t57210 = t14101 * t12080;
    let t57212 = -t55607 + t54355 - t55609 + t54378 - t57195 / 384.0 - t57197 / 192.0 - t57199 / 192.0 - t55623 + 7.0 / 288.0 * t57201 + t57204 / 24.0 - 7.0 / 288.0 * t57206 + t57208 / 24.0 + t57210 / 16.0;
    (t57212,)
}
