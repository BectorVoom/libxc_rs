//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 562/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk562<F: Float>(t311: F, t3293: F, t3297: F, t2580: F, t3012: F, t2578: F, t191: F, t932: F, t2572: F, t3288: F, t1068: F, t869: F) -> (F, F, F, F, F, F, F) {
    let t3298 = t311 * t3293 * t3297;
    let t3300 = t3012 * t2580;
    let t3301 = t2578 * t3300;
    let t3303 = t932 * t191;
    let t3304 = t3288 * t2572;
    let t3305 = t3303 * t3304;
    let t3307 = t869 * t1068;
    (t3298, t3300, t3301, t3303, t3304, t3305, t3307)
}
