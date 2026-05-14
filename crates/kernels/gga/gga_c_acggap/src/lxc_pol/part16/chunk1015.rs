//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1015/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1015<F: Float>(t1181: F, t26554: F, t7351: F, t7426: F, t2068: F, t8480: F, t8778: F, t26214: F, t604: F, t30543: F, t9653: F, t5532: F, t7564: F, t8600: F, t2288: F, t5720: F) -> (F, F, F, F, F, F) {
    let t39876 = t7426 * t1181 * t7351 * t26554;
    let t39879 = t2068 * t8480 * t8778;
    let t39883 = t2068 * t1181 * t604 * t26214;
    let t39885 = t30543 * t9653;
    let t39889 = t7564 * t1181 * t8600 * t5532;
    let t39891 = t2288 * t5720;
    (t39876, t39879, t39883, t39885, t39889, t39891)
}
