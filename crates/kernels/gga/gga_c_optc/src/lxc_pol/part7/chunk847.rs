//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 847/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk847<F: Float>(t2669: F, t7373: F, t2679: F, t2606: F, t8002: F, t8108: F, t2746: F, t298: F, t301: F, t305: F, t8113: F, t19: F, t7380: F) -> (F, F, F, F, F, F, F, F) {
    let t8115 = t2669 * t7373;
    let t8116 = t8115 * t2679;
    let t8119 = t8002 * t2606;
    let t8120 = t8108 * t8119;
    let t8124 = F::new(1.0) / t2746 / t298;
    let t8125 = t8124 * t301;
    let t8126 = t8125 * t305;
    let t8127 = t8126 * t8113;
    let t8128 = t7380 * t19;
    (t8115, t8116, t8120, t8124, t8125, t8126, t8127, t8128)
}
