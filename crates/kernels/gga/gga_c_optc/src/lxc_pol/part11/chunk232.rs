//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 232/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk232<F: Float>(t522: F, t531: F, t182: F, t179: F) -> (F, F, F, F, F) {
    let t720 = -0.19388333333333333333e1 * t522 - 0.12315e-2 * t531;
    let t722 = t182 * t182;
    let t723 = 1.0 / t722;
    let t724 = t179 * t723;
    let t727 = -0.72691666666666666667e3 * t522 - 0.78666666666666666667e2 * t531;
    (t720, t722, t723, t724, t727)
}
