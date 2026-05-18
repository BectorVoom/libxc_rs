//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1118/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1118<F: Float>(t1181: F, t5693: F, t604: F, t8463: F, t5697: F, t7351: F, t7575: F, t1849: F, t322: F, t1165: F, t7493: F, t5608: F, t7561: F) -> (F, F, F, F, F) {
    let t39485 = t8463 * t1181 * t604 * t5693;
    let t39489 = t7575 * t1181 * t7351 * t5697;
    let t39491 = t1849 * t322;
    let t39494 = t7493 * t1165 * t7351 * t39491;
    let t39497 = t7561 * t5608;
    (t39485, t39489, t39491, t39494, t39497)
}
