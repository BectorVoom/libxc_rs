//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 746/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk746<F: Float>(t12669: F, t825: F, t10007: F, t935: F, t9438: F, t2610: F, t3234: F, t2365: F, t2033: F, t959: F, t9817: F, t10033: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12670 = t825 * t12669;
    let t12691 = t10007 * t935;
    let t12692 = t9438 * t12691;
    let t12693 = t825 * t12692;
    let t12695 = t2610 * t3234;
    let t12696 = t2365 * t12695;
    let t12697 = t2033 * t12696;
    let t12699 = t9817 * t959;
    let t12701 = t10033 * t959;
    (t12670, t12691, t12692, t12693, t12695, t12696, t12697, t12699, t12701)
}
