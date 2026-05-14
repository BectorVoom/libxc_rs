//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 276/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk276<F: Float>(t1085: F, t406: F, t1023: F, t1049: F, t414: F) -> (F, F, F, F) {
    let t1086 = t406 * t1085;
    let t1088 = 0.301925e0 * t1023;
    let t1091 = 0.82785e-1 * t1049;
    let t1094 = 1.0 / t414;
    (t1086, t1088, t1091, t1094)
}
