//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1223/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1223<F: Float>(t162: F, t56139: F, t127: F, t1271: F, t16221: F, t22166: F, t13214: F, t4649: F, t16370: F, t16287: F, t2034: F, t4623: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t56140 = t162 * t56139;
    let t56144 = t16221 * t1271 * t127;
    let t56145 = t22166 * t56144;
    let t56148 = t13214 * t4649;
    let t56149 = t162 * t56148;
    let t56153 = t16370 * t1271 * t127;
    let t56154 = t162 * t56153;
    let t56158 = t16287 * t1271 * t127;
    let t56159 = t2034 * t56158;
    let t56164 = t4623 * t4623;
    (t56140, t56144, t56145, t56148, t56149, t56153, t56154, t56158, t56159, t56164)
}
