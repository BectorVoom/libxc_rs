//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1245/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1245<F: Float>(t3127: F, t9123: F, t3234: F, t9040: F, t9189: F, t3244: F, t9058: F, t9142: F, t1113: F, t530: F, t3237: F, t11899: F, t3105: F, t1179: F, t27226: F, t27001: F) -> (F, F, F, F, F, F, F, F) {
    let t27493 = t9123 * t3127;
    let t27510 = t3234 * t9189 * t9040;
    let t27513 = t3244 * t9142 * t9058;
    let t27515 = t530 * t1113;
    let t27517 = t3244 * t27515 * t3237;
    let t27528 = t3244 * t9142 * t9040;
    let t27533 = t11899 * t3105;
    let t27537 = t1179 * t27226;
    let t27541 = t1179 * t27001;
    (t27493, t27510, t27513, t27517, t27528, t27533, t27537, t27541)
}
