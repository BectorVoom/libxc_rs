//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1110/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1110<F: Float>(t1181: F, t604: F, t6203: F, t7575: F, t1165: F, t6209: F, t7351: F, t20417: F, t2068: F, t2073: F, t31346: F, t5932: F) -> (F, F, F, F) {
    let t39382 = t7575 * t1181 * t604 * t6203;
    let t39386 = t7575 * t1165 * t7351 * t6209;
    let t39389 = t2068 * t20417 * t2073;
    let t39391 = t31346 * t5932;
    (t39382, t39386, t39389, t39391)
}
