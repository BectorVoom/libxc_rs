//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1220/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1220<F: Float>(t1846: F, t3476: F, t1008: F, t6220: F, t6228: F, t384: F, t398: F, t4623: F, t535: F, t1111: F, t1165: F, t20400: F, t3361: F) -> (F, F, F, F, F) {
    let t22325 = t3476 * t1846;
    let t22327 = t1008 * t6220;
    let t22329 = t1008 * t6228;
    let t22333 = t384 * t398 * t535 * t4623;
    let t22337 = t3361 * t1165 * t20400 * t1111;
    (t22325, t22327, t22329, t22333, t22337)
}
