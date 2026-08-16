//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 957/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk957<F: Float>(t3237: F, t9142: F, t3244: F, t2367: F, t3093: F, t1162: F, t8538: F, t914: F, t1179: F, t8505: F, t8521: F, t3126: F, t9073: F) -> (F, F, F, F, F, F) {
    let t9143 = t9142 * t3237;
    let t9144 = t3244 * t9143;
    let t9148 = t2367 * t3093;
    let t9149 = t1162 * t9148;
    let t9151 = t914 * t8538;
    let t9156 = t1179 * t8505;
    let t9158 = t1179 * t8521;
    let t9160 = t9073 * t3126;
    (t9144, t9149, t9151, t9156, t9158, t9160)
}
