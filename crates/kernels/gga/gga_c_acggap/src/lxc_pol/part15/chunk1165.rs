//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1165/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1165<F: Float>(t1886: F, t7614: F, t1901: F, t7605: F, t2001: F, t6102: F, t6241: F, t7822: F, t6245: F, t6249: F, t6148: F, t30219: F, t9720: F) -> (F, F, F, F, F, F, F, F) {
    let t40170 = t7614 * t1886;
    let t40172 = t7605 * t1901;
    let t40174 = t2001 * t6102;
    let t40179 = t7822 * t6241;
    let t40181 = t7822 * t6245;
    let t40183 = t7822 * t6249;
    let t40185 = t7822 * t6148;
    let t40187 = t30219 * t9720;
    (t40170, t40172, t40174, t40179, t40181, t40183, t40185, t40187)
}
