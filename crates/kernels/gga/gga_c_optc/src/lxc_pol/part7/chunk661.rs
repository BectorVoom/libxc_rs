//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 661/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk661<F: Float>(t517: F, t6369: F, t11: F, t2: F, t39: F, t1776: F, t6366: F, t525: F, t3649: F, t3696: F, t6364: F, t6367: F, t572: F, t1824: F, t564: F, t62: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6370 = t517 * t6369;
    let t6373 = 1.0/pow_3_2(t11);
    let t6374 = t6373 * t2;
    let t6375 = t6374 * t39;
    let t6377 = t1776 * t6366;
    let t6379 = t525 * t6369;
    let t6382 = -0.47063e1 * t6364 + 0.31375333333333333334e1 * t6367 - 0.36604555555555555556e1 * t6370 - 0.16068111111111111111e1 * t3649 + 0.28051666666666666666e0 * t6375 - 0.56103333333333333332e0 * t6377 - 0.6545388888888888889e0 * t6379 - 0.46308888888888888888e0 * t3696;
    let t6383 = t6382 * t572;
    let t6387 = 1.0 / t1824 / t564;
    let t6388 = t62 * t6387;
    (t6370, t6374, t6375, t6377, t6379, t6382, t6383, t6387, t6388)
}
