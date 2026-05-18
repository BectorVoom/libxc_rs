//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 861/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk861<F: Float>(t572: F, t605: F, t188: F, t174: F, t838: F, t190: F, t25: F, t2718: F, t4941: F, t4943: F, t4945: F, t4947: F, t5044: F, t5241: F, t5271: F, t7327: F, t7335: F, t7337: F, t7342: F, t7347: F, t7351: F, t7356: F, t7360: F, t7364: F) -> (F, F) {
    let t7365 = t605 * t572;
    let t7369 = t188 * t572;
    let t7371 = t174 * t838 * t7369;
    let t7373 = F::new(0.13333333333333333333e-1) * t25 * t7327 - F::new(0.31992592592592592592e-1) * t4941 + F::new(0.7998148148148148148e-2) * t4943 - F::new(0.23994444444444444444e-1) * t4945 + F::new(0.11997222222222222222e-1) * t4947 - t5241 + t7335 - F::new(0.22222222222222222222e-2) * t25 * t7337 - F::new(0.29629629629629629629e-2) * t25 * t7342 - F::new(0.88888888888888888887e-2) * t2718 * t7347 + F::new(0.13333333333333333333e-1) * t25 * t7351 + F::new(0.53333333333333333332e-1) * t2718 * t7356 - F::new(0.39999999999999999999e-1) * t25 * t7360 - t7364 - F::new(0.13333333333333333333e-1) * t190 * t5044 * t7365 - F::new(0.71983333333333333334e-1) * t7371 - t5271;
    (t7371, t7373)
}
