//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 611/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk611<F: Float>(t11576: F, t739: F, t738: F, t169: F, t299: F, t706: F, t3645: F, t702: F, t3614: F, t795: F) -> (F, F, F, F, F, F) {
    let t11577 = t739 * t11576;
    let t11578 = t738 * t11577;
    let t11588 = t11576 * t169 * t299;
    let t11589 = t706 * t11588;
    let t11592 = t3645 * t702;
    let t11595 = t795 * t3614;
    (t11577, t11578, t11588, t11589, t11592, t11595)
}
