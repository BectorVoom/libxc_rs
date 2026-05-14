//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 538/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk538<F: Float>(t3343: F, t3345: F, t1026: F, t933: F, t937: F, t1081: F, t954: F, t969: F, t1936: F, t325: F, t943: F, t3056: F, t122: F, t761: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3346 = t3343 * t3345;
    let t3348 = t933 * t1026;
    let t3349 = t3348 * t937;
    let t3351 = t1081 * t954;
    let t3355 = t1081 * t969;
    let t3357 = t325 * t1936;
    let t3358 = t3357 * t943;
    let t3360 = t325 * t3056;
    let t3361 = t3360 * t943;
    let t3363 = t761 * t122;
    (t3346, t3348, t3349, t3351, t3355, t3357, t3358, t3360, t3361, t3363)
}
