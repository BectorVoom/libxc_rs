//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1151/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1151<F: Float>(t13287: F, t31057: F, t38857: F, t1181: F, t5651: F, t599: F, t8463: F, t5572: F, t7351: F, t7575: F, t2016: F, t9618: F) -> (F, F, F, F) {
    let t39914 = t31057 * t13287 * t38857;
    let t39919 = t8463 * t1181 * t599 * t5651;
    let t39923 = t7575 * t1181 * t7351 * t5572;
    let t39925 = t2016 * t9618;
    (t39914, t39919, t39923, t39925)
}
