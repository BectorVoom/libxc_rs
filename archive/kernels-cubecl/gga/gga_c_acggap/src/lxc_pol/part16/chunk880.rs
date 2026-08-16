//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 880/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk880<F: Float>(t1181: F, t3754: F, t604: F, t7426: F, t1170: F, t2066: F, t592: F, t7777: F, t2070: F, t1165: F, t3759: F, t7351: F) -> (F, F, F, F) {
    let t30452 = t7426 * t1181 * t604 * t3754;
    let t30456 = t1170 * t592 * t7777 * t2066;
    let t30457 = t30456 * t2070;
    let t30463 = t7426 * t1165 * t7351 * t3759;
    (t30452, t30456, t30457, t30463)
}
