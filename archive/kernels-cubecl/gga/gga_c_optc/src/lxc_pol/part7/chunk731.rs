//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 731/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk731<F: Float>(t5: F, t6877: F, t6879: F, t675: F, t2024: F, t6888: F, t696: F, t2164: F, t2174: F, t155: F, t2157: F, t652: F) -> (F, F, F, F, F, F) {
    let t7003 = t5 * t6877;
    let t7004 = t7003 * t6879;
    let t7005 = t675 * t7004;
    let t7008 = t7003 * t2024;
    let t7009 = t675 * t7008;
    let t7012 = t696 * t6888;
    let t7015 = t2164 * t2174;
    let t7018 = t155 * t2157 * t652;
    (t7003, t7005, t7009, t7012, t7015, t7018)
}
