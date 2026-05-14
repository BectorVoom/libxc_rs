//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1417/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1417<F: Float>(t25462: F, t31340: F, t28960: F, t6963: F, t1476: F, t5422: F, t10697: F, t115024: F, t128004: F, t1466: F, t1477: F, t193: F, t19883: F, t24964: F, t2665: F, t28987: F, t29000: F, t29008: F, t29024: F, t29042: F, t31679: F, t31686: F, t31936: F, t3746: F, t4135: F, t4299: F, t44601: F, t6210: F, t6216: F, t684: F, t7114: F, t7129: F, t875: F) -> (F,) {
    let t128723 = t25462 * t31340;
    let t128725 = t6963 * t28960;
    let t128748 = t1476 * t5422;
    let t128755 = -t115024 + 2.0 / 9.0 * t29000 * t2665 * t29024 * t3746 + t128723 / 27.0 - t128725 / 9.0 + t6963 * t29042 / 3.0 - t6210 * t31679 / 3.0 - 24.0 * t10697 * t7114 * t4299 - 2.0 / 3.0 * t1466 * t193 * t24964 * t31686 - 2.0 * t4135 * t7129 - 4.0 * t128004 + 48.0 * t44601 * t31936 * t875 + t1466 * t193 * t1477 * t19883 / 6.0 - t6216 * t2665 * t128748 * t684 / 18.0 - t29008 * t28987 / 9.0;
    (t128755,)
}
