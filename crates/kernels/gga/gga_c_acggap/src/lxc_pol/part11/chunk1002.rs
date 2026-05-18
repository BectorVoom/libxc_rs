//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1002/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1002<F: Float>(t119: F, t8993: F, t1181: F, t5258: F, t604: F, t7575: F, t1165: F, t4930: F, t7351: F, t1432: F, t30147: F, t30148: F, t7842: F) -> (F, F, F, F) {
    let t33818 = t119 * t8993;
    let t33823 = t7575 * t1181 * t604 * t5258;
    let t33827 = t7575 * t1165 * t7351 * t4930;
    let t33831 = t30147 * t7842 * t30148 * t1432;
    (t33818, t33823, t33827, t33831)
}
