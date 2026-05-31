//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1371/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1371<F: Float>(t15431: F, t9270: F, t15482: F, t2376: F, t2408: F, t2409: F, t2494: F, t26654: F, t3066: F, t3067: F, t3068: F, t3721: F, t4110: F, t4207: F, t4227: F, t53750: F, t54598: F, t54599: F, t55351: F, t56776: F, t56783: F, t56787: F, t56791: F, t56793: F, t56799: F, t56811: F, t56813: F, t9296: F, t938: F) -> F {
    let t58431 = t9270 * t15431;
    let t58444 = t56776 / F::cast_from(12.0_f64) - t55351 - t56783 / F::cast_from(24.0_f64) + t56787 / F::cast_from(768.0_f64) - t56791 / F::cast_from(192.0_f64) - t56793 / F::cast_from(48.0_f64) + t54598 * t54599 * t4207 * t3068 / F::cast_from(4.0_f64) + t56799 / F::cast_from(24.0_f64) + t3066 * t2409 * t3067 * t15482 * t938 / F::cast_from(48.0_f64) - t3066 * t2409 * t9296 * t4110 * t3721 / F::cast_from(16.0_f64) + F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t58431 + t2408 * t2409 * t26654 * t4207 / F::cast_from(24.0_f64) + t2408 * t2409 * t2376 * t4227 * t2494 / F::cast_from(24.0_f64) - t53750 - t56811 / F::cast_from(384.0_f64) + t56813 / F::cast_from(12.0_f64);
    t58444
}
