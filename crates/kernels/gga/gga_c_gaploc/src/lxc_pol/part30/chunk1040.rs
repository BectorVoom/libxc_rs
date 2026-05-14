//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1040/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1040<F: Float>(t31216: F, t1429: F, t2365: F, t2366: F, t6393: F, t21074: F, t901: F, t20675: F, t9538: F, t1406: F, t6575: F, t9264: F, t2349: F, t2482: F, t9263: F, t165: F, t4324: F, t874: F) -> (F, F, F, F, F, F, F, F) {
    let t31217 = 0.2044956050875773316e1 * t31216;
    let t31291 = 0.29792074959875355558e-1 * t1429 * t2365 * t2366 * t6393;
    let t31299 = 0.29792074959875355558e-1 * t21074 * t901;
    let t31346 = t20675 * t9538;
    let t31347 = 0.76685851907841499352e0 * t31346;
    let t31356 = t1406 * t6575;
    let t31357 = t31356 * t9264;
    let t31358 = 0.1533717038156829987e1 * t31357;
    let t31360 = t9263 * t2349 * t2482;
    let t31361 = 0.1533717038156829987e1 * t31360;
    let t31379 = t165 * t874 * t4324;
    (t31217, t31291, t31299, t31347, t31356, t31358, t31361, t31379)
}
