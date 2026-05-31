//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1029/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1029<F: Float>(t3070: F, t9270: F, t1115: F, t2397: F, t2401: F, t2408: F, t2498: F, t3207: F, t335: F, t4402: F, t6175: F, t6731: F, t833: F, t844: F, t9215: F, t9220: F, t9224: F, t9228: F, t9232: F, t9236: F, t9241: F, t9243: F, t9249: F, t9253: F, t9255: F, t9260: F, t9265: F) -> F {
    let t9272 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t9270 * t3070;
    let t9273 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t6175 - t3207 * t9215 / F::cast_from(16.0_f64) - t2408 * t9220 / F::cast_from(24.0_f64) - t335 * t9224 / F::cast_from(96.0_f64) + t2401 * t9228 / F::cast_from(16.0_f64) + t335 * t9232 / F::cast_from(48.0_f64) - t844 * t9236 / F::cast_from(48.0_f64) - t9241 * t9243 / F::cast_from(4.0_f64) - t9249 - t1115 * t4402 / F::cast_from(96.0_f64) + t9253 - t6731 - t335 * t9255 / F::cast_from(48.0_f64) - t844 * t9260 / F::cast_from(48.0_f64) + t9265 * t833 / F::cast_from(96.0_f64) + t2498 * t2397 / F::cast_from(48.0_f64) - t9272;
    t9273
}
