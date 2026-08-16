//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 617/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk617<F: Float>(t816: F, t820: F, t3258: F, t3257: F, t2277: F, t2290: F, t2312: F, t3110: F, t3115: F, t3118: F, t3122: F, t3127: F, t3182: F, t3186: F, t3247: F, t3249: F, t3253: F) -> (F, F, F, F) {
    let t3259 = t816 * t820;
    let t3260 = t3258 * t3259;
    let t3261 = t3257 * t3260;
    let t3265 = t3247 * t3249 / F::cast_from(512.0_f64) - t3118 + t3110 - t2312 * t3253 / F::cast_from(384.0_f64) + t2277 * t3261 / F::cast_from(768.0_f64) + t3186 + t3115 + t3122 - t3127 - t3182 + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t2290;
    (t3259, t3260, t3261, t3265)
}
