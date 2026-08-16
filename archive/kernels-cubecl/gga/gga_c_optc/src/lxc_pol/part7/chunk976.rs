//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 976/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk976<F: Float>(t106: F, t664: F, t140: F, t6917: F, t616: F, t645: F, t2029: F, t3500: F, t3466: F, t624: F, t155: F, t6990: F) -> (F, F, F, F, F, F) {
    let t9804 = t106 * t664;
    let t9839 = t6917 * t140;
    let t9870 = t645 * t616;
    let t9896 = t3500 * t2029;
    let t9917 = t3466 * t624;
    let t9954 = t155 * t6990;
    (t9804, t9839, t9870, t9896, t9917, t9954)
}
