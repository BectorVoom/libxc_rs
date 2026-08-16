//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1175/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1175<F: Float>(t1181: F, t5959: F, t604: F, t7575: F, t5964: F, t1859: F, t1992: F, t30154: F, t7586: F, t1164: F, t9685: F, t2068: F, t2069: F) -> (F, F, F, F) {
    let t40196 = t7575 * t1181 * t604 * t5959;
    let t40200 = t7575 * t1181 * t604 * t5964;
    let t40204 = t30154 * t7586 * t1992 * t1859;
    let t40206 = t1164 * t9685;
    let t40208 = t2068 * t40206 * t2069;
    (t40196, t40200, t40204, t40208)
}
