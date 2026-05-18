//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1070/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1070<F: Float>(t7119: F, t9917: F, t2035: F, t21933: F, t6778: F, t7110: F, t7122: F, t7133: F, t6928: F, t115: F, t658: F, t5: F) -> (F, F, F, F, F, F) {
    let t23234 = t9917 * t7119;
    let t23247 = t21933 * t2035;
    let t23254 = t7110 * t6778;
    let t23259 = t7122 * t7133;
    let t23267 = t7110 * t6928;
    let t23269 = t658 * t115;
    let t23270 = t23269 * t5;
    (t23234, t23247, t23254, t23259, t23267, t23270)
}
