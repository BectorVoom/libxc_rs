//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 388/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk388<F: Float>(t188: F, t3701: F, t3695: F, t531: F, t3689: F, t569: F, t568: F, t600: F, t193: F, t3157: F, t3165: F, t3179: F, t3197: F, t3370: F, t3376: F, t3379: F, t3383: F, t3386: F, t3393: F, t3398: F, t3401: F, t3406: F, t3409: F, t3413: F, t557: F, t574: F, t597: F) -> (F, F, F, F, F, F, F) {
    let t3702 = t188 * t3701;
    let t3705 = t531 * t3695;
    let t3709 = t569 * t3689;
    let t3710 = t568 * t3709;
    let t3714 = t600 * t3689;
    let t3715 = t568 * t3714;
    let t3718 = t3370 + 0.35750489951850426669e0 * t3702 * t193 + t3157 - t3379 + t3376 - t3383 - t3165 + t3386 - 0.35750489951850426669e0 * t557 * t3705 - t3393 - 0.38342925953920749677e0 * t3179 + t3401 - 0.23005755572352449806e1 * t574 * t3710 - t3398 - t3406 + t3409 + 0.38342925953920749677e0 * t3197 - t3413 + 0.23005755572352449806e1 * t597 * t3715;
    (t3702, t3705, t3709, t3710, t3714, t3715, t3718)
}
