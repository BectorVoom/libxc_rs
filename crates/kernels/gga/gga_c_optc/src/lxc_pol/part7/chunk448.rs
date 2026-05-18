//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 448/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk448<F: Float>(t1008: F, t2246: F, t1007: F, t23: F, t6: F, t1014: F, t287: F) -> (F, F, F) {
    let t2247 = t1008 * t2246;
    let t2248 = t1007 * t2247;
    let t2251 = t6 * t23;
    let t2253 = t2251 * t287 * t1014;
    (t2247, t2248, t2253)
}
