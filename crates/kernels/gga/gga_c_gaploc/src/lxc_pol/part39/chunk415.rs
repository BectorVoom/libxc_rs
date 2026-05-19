//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 415/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk415<F: Float>(t2487: F, t3407: F, t2854: F, t874: F, t1445: F, t1562: F, t3338: F, t600: F, t568: F, t193: F, t3157: F, t3165: F, t3180: F, t3198: F, t3370: F, t3372: F, t3376: F, t3379: F, t3383: F, t3386: F, t3387: F, t3393: F, t3398: F, t3401: F, t3403: F, t3406: F, t557: F, t574: F, t597: F) -> (F, F, F, F, F, F, F) {
    let t3408 = t2487 * t3407;
    let t3409 = F::cast_from(0.19171462976960374838e0_f64) * t3408;
    let t3410 = t2854 * t874;
    let t3411 = t1445 * t3410;
    let t3413 = F::cast_from(0.69017266717057349418e1_f64) * t1562 * t3411;
    let t3414 = t600 * t3338;
    let t3415 = t568 * t3414;
    let t3418 = t3370 + F::cast_from(0.35750489951850426669e0_f64) * t3372 * t193 + t3376 - t3379 + t3157 - t3165 - t3383 + t3386 - F::cast_from(0.35750489951850426669e0_f64) * t557 * t3387 - t3393 - t3398 + t3401 - F::cast_from(0.23005755572352449806e1_f64) * t574 * t3403 - t3180 - t3406 + t3198 + t3409 - t3413 + F::cast_from(0.23005755572352449806e1_f64) * t597 * t3415;
    (t3409, t3410, t3411, t3413, t3414, t3415, t3418)
}
