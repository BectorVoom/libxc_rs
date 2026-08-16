//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 393/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk393<F: Float>(t169: F, t3431: F, t299: F, t706: F, t1035: F, t2558: F, t943: F, t2936: F, t948: F, t2508: F, t2949: F, t883: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3432 = t3431 * t169;
    let t3433 = t3432 * t299;
    let t3434 = t706 * t3433;
    let t3437 = t1035 * t2558;
    let t3438 = t943 * t3437;
    let t3439 = F::cast_from(0.32043859292259267849e-3_f64) * t3438;
    let t3440 = t2936 * t948;
    let t3442 = F::cast_from(0.23071578690426672851e-1_f64) * t2508 * t3440;
    let t3443 = t883 * t2949;
    (t3432, t3433, t3434, t3437, t3438, t3439, t3440, t3442, t3443)
}
