//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 697/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk697<F: Float>(t1115: F, t2244: F, t2401: F, t2408: F, t2503: F, t3047: F, t3052: F, t3055: F, t3084: F, t3086: F, t3312: F, t3321: F, t335: F, t3724: F, t3733: F, t3739: F, t3744: F, t3889: F, t3893: F, t3898: F, t3903: F, t3909: F, t3913: F, t3917: F, t3921: F, t833: F, t844: F) -> F {
    let t3928 = t335 * t3724 / F::cast_from(48.0_f64) - t1115 * t3052 / F::cast_from(24.0_f64) - t1115 * t3047 / F::cast_from(48.0_f64) - t3055 * t3733 / F::cast_from(96.0_f64) - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t3084 + t2401 * t3739 / F::cast_from(16.0_f64) + t2408 * t3744 / F::cast_from(24.0_f64) + t2244 - t335 * t3889 / F::cast_from(96.0_f64) - t335 * t3893 / F::cast_from(48.0_f64) - t844 * t3898 / F::cast_from(48.0_f64) - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t3321 - t844 * t3903 / F::cast_from(24.0_f64) + t335 * t3909 / F::cast_from(96.0_f64) + t3913 * t833 / F::cast_from(96.0_f64) + t3917 * t833 / F::cast_from(96.0_f64) + t3921 * t833 / F::cast_from(96.0_f64) + t1115 * t2503 / F::cast_from(48.0_f64) + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t3312 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t3086;
    t3928
}
