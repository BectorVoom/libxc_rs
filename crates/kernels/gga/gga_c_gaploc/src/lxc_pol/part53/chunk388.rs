//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 388/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk388<F: Float>(t3216: F, t3226: F, t3218: F, t3223: F, t471: F, t1035: F, t2558: F, t943: F, t2936: F, t948: F, t2508: F, t2949: F, t883: F, t2562: F, t2958: F, t935: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3423 = 3.0 / 128.0 * t3216;
    let t3426 = t3226 / 128.0;
    let t3427 = t3423 - 9.0 / 4096.0 * t3218 + 3.0 / 4096.0 * t3223 - t3426;
    let t3428 = t3427 * t471;
    let t3437 = t1035 * t2558;
    let t3438 = t943 * t3437;
    let t3439 = 0.32043859292259267849e-3 * t3438;
    let t3440 = t2936 * t948;
    let t3442 = 0.23071578690426672851e-1 * t2508 * t3440;
    let t3443 = t883 * t2949;
    let t3444 = t2562 * t3443;
    let t3445 = t943 * t3444;
    let t3446 = 0.32043859292259267849e-3 * t3445;
    let t3447 = t2958 * t935;
    (t3423, t3426, t3427, t3428, t3437, t3439, t3440, t3442, t3444, t3446, t3447)
}
