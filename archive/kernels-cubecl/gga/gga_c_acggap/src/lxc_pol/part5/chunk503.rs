//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 503/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk503<F: Float>(t420: F, t56: F, t137: F, t495: F, t506: F, t6: F, t119: F, t182: F) -> (F, F, F, F) {
    let t2066 = t56 * t420;
    let t2297 = t137 * t495;
    let t2325 = t6 * t506;
    let t2450 = t119 * t182;
    (t2066, t2297, t2325, t2450)
}
