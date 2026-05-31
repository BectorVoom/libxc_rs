//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1362/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1362<F: Float>(t14979: F, t15429: F, t22534: F, t2376: F, t2408: F, t2409: F, t3066: F, t3207: F, t36129: F, t3703: F, t4110: F, t4216: F, t55137: F, t55161: F, t56434: F, t56439: F, t56442: F, t56460: F, t56474: F, t56476: F, t56483: F, t56495: F, t56500: F, t8589: F, t8629: F) -> F {
    let t58172 = -F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t56434 + t56439 / F::cast_from(768.0_f64) + t56442 / F::cast_from(192.0_f64) - t56460 / F::cast_from(384.0_f64) - t55161 - t3066 * t2409 * t22534 * t15429 / F::cast_from(16.0_f64) + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t56474 + t3066 * t2409 * t36129 * t4216 / F::cast_from(24.0_f64) - t3207 * t2409 * t2376 * t4110 * t3703 / F::cast_from(16.0_f64) + t2408 * t2409 * t8589 * t14979 / F::cast_from(24.0_f64) - F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t56476 - t8629 * t55137 / F::cast_from(24.0_f64) - t56483 / F::cast_from(24.0_f64) - t56495 / F::cast_from(48.0_f64) + t56500 / F::cast_from(96.0_f64);
    t58172
}
