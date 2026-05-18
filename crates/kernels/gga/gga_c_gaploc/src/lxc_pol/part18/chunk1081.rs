//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1081/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1081<F: Float>(t540: F, t8071: F, t20550: F, t7892: F, t1: F, t106: F, t192: F, t7861: F, t1564: F, t7905: F, t9448: F, t1397: F, t8247: F) -> (F, F, F, F, F, F) {
    let t26279 = t8071 * t540;
    let t26328 = t20550 * t7892;
    let t26343 = t7861 * t1 * t106 * t192;
    let t26428 = t1564 * t7861;
    let t26435 = t9448 * t7905;
    let t26451 = t1397 * t8247;
    (t26279, t26328, t26343, t26428, t26435, t26451)
}
