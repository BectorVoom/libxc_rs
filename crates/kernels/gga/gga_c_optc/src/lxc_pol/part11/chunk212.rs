//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 212/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk212<F: Float>(t40: F, t592: F, t1: F, t60: F, t508: F, t518: F, t84: F, t75: F) -> (F, F, F, F, F) {
    let t593 = t40 * t592;
    let t596 = t60 * t1;
    let t598 = t518 * t508 * t84;
    let t600 = 0.18311555036753159941e-3 * t596 * t598;
    let t601 = t60 * t75;
    (t593, t596, t598, t600, t601)
}
