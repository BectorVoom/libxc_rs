//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 259/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk259<F: Float>(t780: F, t782: F, t765: F, t214: F, t136: F, t529: F, t222: F, t224: F) -> (F, F, F, F, F, F) {
    let t783 = t780 * t782;
    let t785 = 0.29896666666666666667e0 * t765;
    let t787 = f64::sqrt(t214);
    let t788 = t787 * t782;
    let t790 = t529 * t136;
    let t792 = t222 * t790 * t224;
    (t783, t785, t787, t788, t790, t792)
}
