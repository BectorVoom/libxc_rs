//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1072/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1072<F: Float>(t12005: F, t12009: F, t12015: F, t12021: F, t12025: F, t12031: F, t12034: F, t12038: F, t12040: F, t12047: F, t12050: F, t2253: F, t2277: F, t6579: F, t9645: F, t9658: F) -> F {
    let t12053 = -t9645 - t12005 - t2253 * t12009 / F::cast_from(384.0_f64) - t2253 * t12015 / F::cast_from(768.0_f64) - t2253 * t12021 / F::cast_from(384.0_f64) + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t6579 * t12025 - F::cast_from(119.0_f64) / F::cast_from(1728.0_f64) * t9658 - t12031 + t12034 + t12038 - t12040 + t12047 - t2277 * t12050 / F::cast_from(1536.0_f64);
    t12053
}
