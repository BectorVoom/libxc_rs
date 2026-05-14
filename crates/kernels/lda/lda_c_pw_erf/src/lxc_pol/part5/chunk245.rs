//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 245/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk245<F: Float>(t40: F, t749: F, t748: F, t85: F, t406: F, t739: F, t408: F, t743: F) -> (F, F, F, F, F) {
    let t750 = t40 * t749;
    let t751 = t748 * t85;
    let t752 = 0.019751789702565206 * t751;
    let t753 = t406 * t739;
    let t754 = t408 * t743;
    let t756 = t753 / 3.0 + t754 / 3.0;
    (t750, t752, t753, t754, t756)
}
