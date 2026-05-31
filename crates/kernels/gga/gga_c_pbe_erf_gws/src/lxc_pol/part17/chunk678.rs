//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 678/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk678<F: Float>(t3306: F, t898: F, t353: F, t338: F, t1120: F, t2246: F, t1144: F, t939: F, t1146: F, t840: F, t1115: F, t2244: F, t2368: F, t2379: F, t2397: F, t2408: F, t3090: F, t3094: F, t3099: F, t3103: F, t3202: F, t3207: F, t3209: F, t3214: F, t335: F, t844: F) -> (F, F, F, F, F, F) {
    let t3307 = t898 * t3306;
    let t3308 = t353 * t3307;
    let t3309 = t338 * t3308;
    let t3312 = t2246 * t1120;
    let t3316 = t1144 * t939;
    let t3317 = t338 * t3316;
    let t3321 = t840 * t1146;
    let t3323 = -t844 * t3090 / F::cast_from(48.0_f64) - t844 * t3094 / F::cast_from(48.0_f64) - t844 * t3099 / F::cast_from(48.0_f64) - t335 * t3103 / F::cast_from(96.0_f64) + t335 * t3202 / F::cast_from(96.0_f64) + t3207 * t3209 / F::cast_from(16.0_f64) + t2408 * t3214 / F::cast_from(48.0_f64) + t1115 * t2397 / F::cast_from(96.0_f64) - t335 * t3309 / F::cast_from(96.0_f64) + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t3312 - t1115 * t2379 / F::cast_from(96.0_f64) - t335 * t3317 / F::cast_from(96.0_f64) - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t2368 + t2244 - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t3321;
    (t3307, t3308, t3309, t3316, t3317, t3323)
}
