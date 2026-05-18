//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 620/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk620<F: Float>(t3294: F, t905: F, t1113: F, t2271: F, t1154: F, t2289: F, t2277: F, t2312: F, t3146: F, t3170: F, t3177: F, t3193: F, t3279: F, t3283: F, t3287: F, t3291: F, t902: F, t914: F, t929: F) -> (F, F, F, F, F) {
    let t3295 = t905 * t3294;
    let t3298 = t1113 * t2271;
    let t3299 = t905 * t3298;
    let t3302 = t2289 * t1154;
    let t3304 = F::new(5.0) / F::new(768.0) * t929 * t3279 - t914 * t3283 / F::new(1536.0) + t3177 - t2312 * t3287 / F::new(384.0) - t2277 * t3291 / F::new(1536.0) + t902 * t3295 / F::new(1536.0) + t902 * t3299 / F::new(1536.0) + t3193 - t3170 + t3146 + F::new(7.0) / F::new(2304.0) * t3302;
    (t3295, t3298, t3299, t3302, t3304)
}
