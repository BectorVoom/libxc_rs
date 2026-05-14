//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 536/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk536<F: Float>(t311: F, t3293: F, t3297: F, t2580: F, t3012: F, t2578: F, t191: F, t932: F, t2572: F, t3288: F, t1068: F, t869: F, t322: F, t1069: F, t913: F, t3274: F, t3276: F, t3279: F, t3282: F, t3286: F, t3290: F) -> (F, F, F, F, F) {
    let t3298 = t311 * t3293 * t3297;
    let t3300 = t3012 * t2580;
    let t3301 = t2578 * t3300;
    let t3303 = t932 * t191;
    let t3304 = t3288 * t2572;
    let t3305 = t3303 * t3304;
    let t3307 = t869 * t1068;
    let t3308 = t3307 * t322;
    let t3310 = t1069 * t913;
    let t3312 = -0.1686740451388888889e-5 * t3274 - 0.84540905957968605066e-5 * t3276 + 0.72463633678258804342e-6 * t3279 + 0.61789714048124642274e-4 * t3282 + 0.14492726735651760868e-5 * t3286 - 0.14492726735651760868e-5 * t3290 + 0.73794894748263888892e-6 * t3298 - 0.25340269868817520617e-4 * t3301 - 0.72463633678258804342e-6 * t3305 + 0.13900948042322754167e-2 * t3308 + 0.13900948042322754167e-2 * t3310;
    (t3300, t3303, t3304, t3307, t3312)
}
