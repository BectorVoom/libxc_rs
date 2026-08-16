//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 622/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk622(t3306: f64, t898: f64, t353: f64, t338: f64, t1120: f64, t2246: f64, t1144: f64, t939: f64, t1146: f64, t840: f64, t1115: f64, t2244: f64, t2368: f64, t2379: f64, t2397: f64, t2408: f64, t3090: f64, t3094: f64, t3099: f64, t3103: f64, t3202: f64, t3207: f64, t3209: f64, t3214: f64, t335: f64, t844: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3307 = t898 * t3306;
    let t3308 = t353 * t3307;
    let t3309 = t338 * t3308;
    let t3312 = t2246 * t1120;
    let t3316 = t1144 * t939;
    let t3317 = t338 * t3316;
    let t3321 = t840 * t1146;
    let t3323 = -t844 * t3090 / 48.0_f64 - t844 * t3094 / 48.0_f64 - t844 * t3099 / 48.0_f64 - t335 * t3103 / 96.0_f64 + t335 * t3202 / 96.0_f64 + t3207 * t3209 / 16.0_f64 + t2408 * t3214 / 48.0_f64 + t1115 * t2397 / 96.0_f64 - t335 * t3309 / 96.0_f64 + 7.0_f64 / 144.0_f64 * t3312 - t1115 * t2379 / 96.0_f64 - t335 * t3317 / 96.0_f64 - 7.0_f64 / 288.0_f64 * t2368 + t2244 - 7.0_f64 / 288.0_f64 * t3321;
    (t3307, t3308, t3309, t3312, t3316, t3317, t3321, t3323)
}
