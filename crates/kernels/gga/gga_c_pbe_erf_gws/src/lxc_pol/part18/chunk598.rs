//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 598/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk598<F: Float>(t3306: F, t898: F, t353: F, t338: F, t1120: F, t2246: F, t1144: F, t939: F, t1146: F, t840: F, t1115: F, t2244: F, t2368: F, t2379: F, t2397: F, t2408: F, t3090: F, t3094: F, t3099: F, t3103: F, t3202: F, t3207: F, t3209: F, t3214: F, t335: F, t844: F) -> (F, F, F, F, F, F, F, F) {
    let t3307 = t898 * t3306;
    let t3308 = t353 * t3307;
    let t3309 = t338 * t3308;
    let t3312 = t2246 * t1120;
    let t3316 = t1144 * t939;
    let t3317 = t338 * t3316;
    let t3321 = t840 * t1146;
    let t3323 = -t844 * t3090 / 48.0 - t844 * t3094 / 48.0 - t844 * t3099 / 48.0 - t335 * t3103 / 96.0 + t335 * t3202 / 96.0 + t3207 * t3209 / 16.0 + t2408 * t3214 / 48.0 + t1115 * t2397 / 96.0 - t335 * t3309 / 96.0 + 7.0 / 144.0 * t3312 - t1115 * t2379 / 96.0 - t335 * t3317 / 96.0 - 7.0 / 288.0 * t2368 + t2244 - 7.0 / 288.0 * t3321;
    (t3307, t3308, t3309, t3312, t3316, t3317, t3321, t3323)
}
