//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 573/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk573<F: Float>(t3414: F, t568: F, t193: F, t3157: F, t3165: F, t3180: F, t3198: F, t3370: F, t3372: F, t3376: F, t3379: F, t3383: F, t3386: F, t3387: F, t3393: F, t3398: F, t3401: F, t3403: F, t3406: F, t3409: F, t3413: F, t557: F, t574: F, t597: F) -> (F, F) {
    let t3415 = t568 * t3414;
    let t3418 = t3370 + 0.35750489951850426669e0 * t3372 * t193 + t3376 - t3379 + t3157 - t3165 - t3383 + t3386 - 0.35750489951850426669e0 * t557 * t3387 - t3393 - t3398 + t3401 - 0.23005755572352449806e1 * t574 * t3403 - t3180 - t3406 + t3198 + t3409 - t3413 + 0.23005755572352449806e1 * t597 * t3415;
    (t3415, t3418)
}
