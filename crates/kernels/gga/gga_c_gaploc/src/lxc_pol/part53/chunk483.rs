//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 483/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk483<F: Float>(t188: F, t9189: F, t3085: F, t4130: F, t590: F, t1339: F, t3116: F, t3196: F, t7014: F, t2488: F, t9278: F, t2487: F, t2344: F, t2465: F, t2464: F, t1641: F, t3193: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9351 = t188 * t9189;
    let t9354 = t4130 * t3085;
    let t9355 = t9354 * t590;
    let t9358 = t1339 * t3116;
    let t9359 = t9358 * t590;
    let t9362 = t7014 * t3196;
    let t9363 = 0.38342925953920749676e0 * t9362;
    let t9364 = t2488 * t9278;
    let t9365 = t2487 * t9364;
    let t9366 = 0.38342925953920749676e0 * t9365;
    let t9367 = t2465 * t2344;
    let t9368 = t2464 * t9367;
    let t9369 = t2487 * t9368;
    let t9370 = 0.85206502119823888169e-1 * t9369;
    let t9371 = t1641 * t3193;
    (t9351, t9354, t9355, t9359, t9362, t9363, t9365, t9366, t9369, t9370, t9371)
}
