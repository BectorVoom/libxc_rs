//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1081/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1081<F: Float>(t1181: F, t38766: F, t604: F, t7413: F, t38771: F, t599: F, t5618: F, t7561: F, t1165: F, t25941: F, t7337: F, t31428: F, t9614: F) -> (F, F, F, F, F) {
    let t38990 = t7413 * t1181 * t604 * t38766;
    let t38994 = t7413 * t1181 * t599 * t38771;
    let t38996 = t7561 * t5618;
    let t39000 = t7337 * t1165 * t604 * t25941;
    let t39002 = t31428 * t9614;
    (t38990, t38994, t38996, t39000, t39002)
}
