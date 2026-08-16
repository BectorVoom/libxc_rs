//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1709/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1709<F: Float>(t11852: F, t66: F, t11145: F, t247: F, t3298: F, t994: F, t4891: F) -> (F, F, F, F) {
    let t11853 = t66 * t11852;
    let t11855 = t247 * t11853 * t11145;
    let t11858 = t994 * t3298;
    let t11859 = t11858 * t4891;
    (t11853, t11855, t11858, t11859)
}
