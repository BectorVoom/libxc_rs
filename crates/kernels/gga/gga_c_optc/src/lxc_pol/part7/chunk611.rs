//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 611/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk611<F: Float>(t3106: F, t3109: F, t1124: F, t1128: F, t1121: F, t2639: F, t465: F) -> (F, F, F) {
    let t3110 = t3106 * t3109;
    let t3113 = t1128 * t1124;
    let t3114 = t1121 * t3113;
    let t3116 = t465 * t2639;
    (t3110, t3114, t3116)
}
