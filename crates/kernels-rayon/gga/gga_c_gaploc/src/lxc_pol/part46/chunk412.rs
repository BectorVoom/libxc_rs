//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 412/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk412(t2487: f64, t3407: f64, t2854: f64, t874: f64, t1445: f64, t1562: f64, t3338: f64, t600: f64, t568: f64, t193: f64, t3157: f64, t3165: f64, t3180: f64, t3198: f64, t3370: f64, t3372: f64, t3376: f64, t3379: f64, t3383: f64, t3386: f64, t3387: f64, t3393: f64, t3398: f64, t3401: f64, t3403: f64, t3406: f64, t557: f64, t574: f64, t597: f64) -> (f64, f64, f64, f64, f64) {
    let t3408 = t2487 * t3407;
    let t3409 = 0.19171462976960374838e0_f64 * t3408;
    let t3410 = t2854 * t874;
    let t3411 = t1445 * t3410;
    let t3413 = 0.69017266717057349418e1_f64 * t1562 * t3411;
    let t3414 = t600 * t3338;
    let t3415 = t568 * t3414;
    let t3418 = t3370 + 0.35750489951850426669e0_f64 * t3372 * t193 + t3376 - t3379 + t3157 - t3165 - t3383 + t3386 - 0.35750489951850426669e0_f64 * t557 * t3387 - t3393 - t3398 + t3401 - 0.23005755572352449806e1_f64 * t574 * t3403 - t3180 - t3406 + t3198 + t3409 - t3413 + 0.23005755572352449806e1_f64 * t597 * t3415;
    (t3410, t3411, t3414, t3415, t3418)
}
