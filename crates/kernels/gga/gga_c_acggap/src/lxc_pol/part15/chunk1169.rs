//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1169/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1169<F: Float>(t1165: F, t2068: F, t39596: F, t7351: F, t31350: F, t5737: F, t7337: F, t8480: F, t8902: F, t30698: F, t38789: F, t604: F) -> (F, F, F, F) {
    let t40241 = t2068 * t1165 * t7351 * t39596;
    let t40243 = t31350 * t5737;
    let t40246 = t7337 * t8480 * t8902;
    let t40251 = t30698 * t1165 * t604 * t38789;
    (t40241, t40243, t40246, t40251)
}
