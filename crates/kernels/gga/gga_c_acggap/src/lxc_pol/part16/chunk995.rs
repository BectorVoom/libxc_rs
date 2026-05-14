//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 995/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk995<F: Float>(t2068: F, t4680: F, t9636: F, t1181: F, t599: F, t6069: F, t2041: F, t5590: F, t5594: F, t1165: F, t5645: F, t604: F, t8463: F, t31362: F, t9589: F, t7337: F, t9588: F) -> (F, F, F, F, F, F, F) {
    let t39551 = t2068 * t4680 * t9636;
    let t39555 = t2068 * t1181 * t599 * t6069;
    let t39557 = t2041 * t5590;
    let t39559 = t2041 * t5594;
    let t39563 = t8463 * t1165 * t604 * t5645;
    let t39567 = t31362 * t9589;
    let t39570 = t7337 * t4680 * t9588;
    (t39551, t39555, t39557, t39559, t39563, t39567, t39570)
}
