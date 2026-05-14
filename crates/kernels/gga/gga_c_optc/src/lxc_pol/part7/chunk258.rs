//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 258/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk258<F: Float>(t232: F, t774: F, t228: F, t216: F, t217: F, t765: F, t772: F) -> (F, F, F, F, F, F) {
    let t776 = 0.62182e-1 * t774 * t232;
    let t777 = t228 * t228;
    let t778 = 1.0 / t777;
    let t779 = t216 * t778;
    let t780 = 1.0 / t217;
    let t782 = -t765 / 3.0 - t772 / 3.0;
    (t776, t777, t778, t779, t780, t782)
}
