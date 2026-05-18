//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 756/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk756<F: Float>(t1165: F, t1421: F, t604: F, t7493: F, t3220: F, t56: F, t2065: F, t2450: F) -> (F, F, F, F, F) {
    let t8458 = t1165 * t604 * t1421;
    let t8459 = t7493 * t8458;
    let t8461 = t56 * t3220;
    let t8462 = t2065 * t8461;
    let t8463 = t2450 * t8462;
    (t8458, t8459, t8461, t8462, t8463)
}
