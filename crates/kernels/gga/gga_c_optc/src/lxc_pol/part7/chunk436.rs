//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 436/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk436<F: Float>(t131: F, t2156: F, t133: F, t155: F, t2025: F, t696: F, t652: F, t693: F) -> (F, F, F, F) {
    let t2157 = t2156 * t131;
    let t2159 = t155 * t2157 * t133;
    let t2160 = t696 * t2025;
    let t2164 = t155 * t693 * t652;
    (t2157, t2159, t2160, t2164)
}
