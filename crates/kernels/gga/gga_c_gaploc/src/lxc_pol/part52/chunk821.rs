//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 821/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk821<F: Float>(t1: F, t3689: F, t544: F, t594: F, t12092: F, t2478: F, t6583: F, t188: F, t46965: F, t11977: F, t524: F, t13778: F, t2487: F, t6985: F, t12078: F, t1415: F, t7030: F) -> (F, F, F, F, F, F) {
    let t48171 = t544 * t594 * t3689 * t1;
    let t48178 = t6583 * t12092 * t2478;
    let t48187 = t188 * t46965;
    let t48190 = t524 * t11977;
    let t48194 = t2487 * t6985 * t13778;
    let t48208 = t1415 * t12078 * t7030;
    (t48171, t48178, t48187, t48190, t48194, t48208)
}
