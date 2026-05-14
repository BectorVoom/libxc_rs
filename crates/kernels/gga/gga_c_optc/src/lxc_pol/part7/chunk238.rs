//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 238/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk238<F: Float>(t647: F, t696: F, t136: F, t652: F, t162: F, t159: F, t133: F, t155: F, t158: F) -> (F, F, F) {
    let t697 = t696 * t647;
    let t700 = t652 * t136;
    let t701 = t700 * t162;
    let t703 = 0.35266493120854938101e-1 * t159 * t701;
    let t705 = t155 * t158 * t133;
    (t697, t703, t705)
}
