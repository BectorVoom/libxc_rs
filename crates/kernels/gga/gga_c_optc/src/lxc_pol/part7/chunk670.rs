//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 670/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk670<F: Float>(t6461: F, t6523: F, t60: F, t40: F, t1948: F, t729: F, t108: F, t176: F, t203: F, t47: F, t768: F, t1885: F, t549: F) -> (F, F, F, F, F, F, F) {
    let t6524 = t6461 + t6523;
    let t6525 = t60 * t6524;
    let t6526 = t40 * t6525;
    let t6527 = t729 * t1948;
    let t6529 = t176 * t6527 * t108;
    let t6530 = t6529 * t203;
    let t6533 = 1.0 / t47 / t768;
    let t6534 = t1885 * t549;
    (t6524, t6525, t6526, t6529, t6530, t6533, t6534)
}
