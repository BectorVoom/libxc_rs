//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1212/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1212<F: Float>(t2583: F, t7845: F, t7848: F, t7894: F, t874: F, t877: F, t2590: F, t7878: F, t893: F, t2597: F, t2586: F, t7871: F) -> (F, F, F, F, F, F, F, F) {
    let t25020 = t2583 * t7845;
    let t25022 = t2583 * t7848;
    let t25025 = t874 * t7894 * t877;
    let t25029 = t7878 * t2590;
    let t25030 = t893 * t25029;
    let t25032 = t7878 * t2597;
    let t25033 = t893 * t25032;
    let t25035 = t2586 * t7871;
    (t25020, t25022, t25025, t25029, t25030, t25032, t25033, t25035)
}
