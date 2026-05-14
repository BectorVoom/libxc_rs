//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1017/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1017<F: Float>(t2030: F, t507: F, t8816: F, t1488: F, t2060: F, t2317: F, t13287: F, t31057: F, t38857: F, t1181: F, t5651: F, t599: F, t8463: F, t5572: F, t7351: F, t7575: F) -> (F, F, F, F, F) {
    let t39907 = t2030 * t507 * t8816;
    let t39910 = t2060 * t1488 * t2317;
    let t39914 = t31057 * t13287 * t38857;
    let t39919 = t8463 * t1181 * t599 * t5651;
    let t39923 = t7575 * t1181 * t7351 * t5572;
    (t39907, t39910, t39914, t39919, t39923)
}
