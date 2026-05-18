//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1137/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1137<F: Float>(t1181: F, t1849: F, t360: F, t7351: F, t7575: F, t604: F, t6209: F, t2060: F, t372: F, t8927: F, t9563: F, t5694: F, t8806: F) -> (F, F, F, F) {
    let t39720 = t7575 * t1181 * t7351 * t1849 * t360;
    let t39724 = t7575 * t1181 * t604 * t6209;
    let t39733 = t2060 * t8927 * t9563 * t372;
    let t39735 = t8806 * t5694;
    (t39720, t39724, t39733, t39735)
}
