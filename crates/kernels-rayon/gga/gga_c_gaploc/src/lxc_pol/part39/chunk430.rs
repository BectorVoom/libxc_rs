//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 430/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk430(t188: f64, t3701: f64, t3695: f64, t531: f64, t3689: f64, t569: f64, t568: f64, t600: f64, t193: f64, t3157: f64, t3165: f64, t3179: f64, t3197: f64, t3370: f64, t3376: f64, t3379: f64, t3383: f64, t3386: f64, t3393: f64, t3398: f64, t3401: f64, t3406: f64, t3409: f64, t3413: f64, t557: f64, t574: f64, t597: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3702 = t188 * t3701;
    let t3705 = t531 * t3695;
    let t3709 = t569 * t3689;
    let t3710 = t568 * t3709;
    let t3714 = t600 * t3689;
    let t3715 = t568 * t3714;
    let t3718 = t3370 + 0.35750489951850426669e0_f64 * t3702 * t193 + t3157 - t3379 + t3376 - t3383 - t3165 + t3386 - 0.35750489951850426669e0_f64 * t557 * t3705 - t3393 - 0.38342925953920749677e0_f64 * t3179 + t3401 - 0.23005755572352449806e1_f64 * t574 * t3710 - t3398 - t3406 + t3409 + 0.38342925953920749677e0_f64 * t3197 - t3413 + 0.23005755572352449806e1_f64 * t597 * t3715;
    (t3702, t3705, t3709, t3710, t3714, t3715, t3718)
}
