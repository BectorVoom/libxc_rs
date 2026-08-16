//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 673/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk673<F: Float>(t1122: F, t3107: F, t1: F, t3209: F, t426: F, t3883: F) -> (F, F, F, F) {
    let t4436 = t3107 * t1122;
    let t4437 = t4436 * t1;
    let t4456 = t3209 * t426;
    let t4457 = t4456 * t3883;
    (t4436, t4437, t4456, t4457)
}
