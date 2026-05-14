//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 663/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk663<F: Float>(t1115: F, t2244: F, t2401: F, t2408: F, t2503: F, t3047: F, t3052: F, t3055: F, t3084: F, t3086: F, t3312: F, t3321: F, t335: F, t3724: F, t3733: F, t3739: F, t3744: F, t3889: F, t3893: F, t3898: F, t3903: F, t3909: F, t3913: F, t3917: F, t3921: F, t833: F, t844: F) -> (F,) {
    let t3928 = t335 * t3724 / 48.0 - t1115 * t3052 / 24.0 - t1115 * t3047 / 48.0 - t3055 * t3733 / 96.0 - 7.0 / 144.0 * t3084 + t2401 * t3739 / 16.0 + t2408 * t3744 / 24.0 + t2244 - t335 * t3889 / 96.0 - t335 * t3893 / 48.0 - t844 * t3898 / 48.0 - 7.0 / 144.0 * t3321 - t844 * t3903 / 24.0 + t335 * t3909 / 96.0 + t3913 * t833 / 96.0 + t3917 * t833 / 96.0 + t3921 * t833 / 96.0 + t1115 * t2503 / 48.0 + 7.0 / 72.0 * t3312 + 7.0 / 144.0 * t3086;
    (t3928,)
}
