//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 414/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk414<F: Float>(t1437: F, t1645: F, t1571: F, t528: F, t1561: F, t565: F, t1559: F, t158: F, t120: F, t19: F, t196: F, t1563: F, t171: F) -> (F, F, F, F, F, F, F) {
    let t4446 = t1645 * t1437;
    let t4507 = t528 * t1571;
    let t4511 = t565 * t1561;
    let t4524 = t1559 * t158;
    let t4525 = t120 * t4524;
    let t4526 = t4525 * t19;
    let t4527 = t196 * t4526;
    let t4529 = 1.0 / t1563 / t171;
    (t4446, t4507, t4511, t4524, t4526, t4527, t4529)
}
