//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 895/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk895<F: Float>(t3109: F, t9129: F, t1179: F, t8508: F, t8946: F, t466: F, t8529: F, t10: F, t1135: F) -> (F, F, F, F, F) {
    let t9176 = t9129 * t3109;
    let t9179 = t1179 * t8508;
    let t9181 = t1179 * t8946;
    let t9188 = 0.22391424203717421017e-2 * t466 * t8529;
    let t9189 = t10 * t1135;
    (t9176, t9179, t9181, t9188, t9189)
}
