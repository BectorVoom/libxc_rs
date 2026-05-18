//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 731/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk731<F: Float>(t3722: F, t779: F, t12214: F, t2580: F, t12259: F, t1901: F, t12161: F, t169: F, t299: F, t706: F, t12250: F, t123: F) -> (F, F, F, F, F, F) {
    let t12291 = t779 * t3722;
    let t12294 = t2580 * t12214;
    let t12297 = t1901 * t12259;
    let t12305 = t12161 * t169 * t299;
    let t12306 = t706 * t12305;
    let t12311 = t12250 * t123;
    (t12291, t12294, t12297, t12305, t12306, t12311)
}
