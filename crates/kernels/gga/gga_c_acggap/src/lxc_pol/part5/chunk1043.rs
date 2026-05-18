//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1043/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1043<F: Float>(t3431: F, t5277: F, t1165: F, t12349: F, t1532: F, t3456: F, t1163: F, t16548: F, t540: F, t12738: F, t5147: F, t1008: F, t5118: F) -> (F, F, F, F, F) {
    let t18037 = t3431 * t5277;
    let t18041 = t3456 * t1165 * t1532 * t12349;
    let t18045 = t1163 * t1165 * t540 * t16548;
    let t18047 = t12738 * t5147;
    let t18062 = t1008 * t5118;
    (t18037, t18041, t18045, t18047, t18062)
}
