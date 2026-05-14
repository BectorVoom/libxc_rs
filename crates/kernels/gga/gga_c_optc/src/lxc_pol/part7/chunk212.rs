//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 212/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk212<F: Float>(t580: F, t75: F, t520: F, t522: F, t526: F, t531: F) -> (F, F) {
    let t581 = t75 * t580;
    let t586 = -0.86308333333333333334e0 * t520 - 0.301925e0 * t522 - 0.5501625e-1 * t526 - 0.82785e-1 * t531;
    (t581, t586)
}
