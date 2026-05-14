//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1072/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1072<F: Float>(t2508: F, t2586: F, t8637: F, t29277: F, t7064: F, t8970: F, t10752: F, t5288: F, t2558: F, t8844: F, t943: F, t25331: F, t2541: F, t25335: F, t7157: F, t10643: F, t7137: F) -> (F, F, F, F, F, F, F) {
    let t32256 = 0.46143157380853345702e-1 * t2508 * t8637 * t2586;
    let t32258 = t7064 * t29277 * t8970;
    let t32259 = 0.1281754371690370714e-2 * t32258;
    let t32266 = 0.46143157380853345702e-1 * t5288 * t10752;
    let t32268 = t943 * t8844 * t2558;
    let t32269 = 0.32043859292259267849e-3 * t32268;
    let t32272 = 0.11535789345213336425e0 * t2508 * t2541 * t25331;
    let t32275 = 0.38452631150711121418e0 * t2508 * t7157 * t25335;
    let t32277 = 0.14355648962932151996e0 * t7137 * t10643;
    (t32256, t32259, t32266, t32269, t32272, t32275, t32277)
}
