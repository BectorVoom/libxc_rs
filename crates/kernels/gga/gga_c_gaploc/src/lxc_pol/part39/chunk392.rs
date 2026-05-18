//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 392/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk392<F: Float>(t169: F, t3234: F, t299: F, t706: F, t2558: F, t954: F, t943: F, t3210: F, t325: F, t738: F, t2571: F, t883: F) -> (F, F, F, F, F, F, F) {
    let t3235 = t3234 * t169;
    let t3236 = t3235 * t299;
    let t3237 = t706 * t3236;
    let t3240 = t954 * t2558;
    let t3242 = F::new(0.64087718584518535698e-3) * t943 * t3240;
    let t3243 = t3210 * t325;
    let t3244 = t738 * t3243;
    let t3247 = t883 * t2571;
    (t3236, t3237, t3240, t3242, t3243, t3244, t3247)
}
