//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 687/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk687<F: Float>(t1111: F, t1165: F, t7351: F, t7426: F, t1964: F, t592: F, t2066: F) -> (F, F, F, F) {
    let t7428 = t1165 * t7351 * t1111;
    let t7429 = t7426 * t7428;
    let t7431 = t592 * t1964;
    let t7432 = t7431 * t2066;
    (t7428, t7429, t7431, t7432)
}
