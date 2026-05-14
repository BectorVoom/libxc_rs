//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 242/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk242<F: Float>(t228: F, t216: F, t217: F, t765: F, t214: F, t136: F, t529: F, t222: F, t224: F) -> (F, F, F, F, F, F, F, F) {
    let t777 = t228 * t228;
    let t778 = 1.0 / t777;
    let t779 = t216 * t778;
    let t780 = 1.0 / t217;
    let t785 = 0.29896666666666666667e0 * t765;
    let t787 = f64::sqrt(t214);
    let t790 = t529 * t136;
    let t792 = t222 * t790 * t224;
    (t777, t778, t779, t780, t785, t787, t790, t792)
}
