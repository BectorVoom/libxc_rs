//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 910/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk910<F: Float>(t987: F, t997: F, t996: F, t2002: F, t6: F, t4: F, t133: F, t5: F, t21: F, t362: F, t7298: F, t2473: F, t7504: F, t322: F, t7253: F, t7256: F) -> (F, F, F, F, F, F, F, F) {
    let t10126 = t997 * t987;
    let t10127 = t996 * t10126;
    let t10194 = t6 * t2002;
    let t10195 = t4 * t10194;
    let t10344 = t5 * t133;
    let t10345 = t21 * t10344;
    let t10615 = t362 * t7298;
    let t10694 = t7504 * t2473;
    let t10825 = t322 * t7253;
    let t10826 = t362 * t7256;
    (t10127, t10194, t10195, t10345, t10615, t10694, t10825, t10826)
}
