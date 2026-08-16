//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 614/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk614<F: Float>(t1449: F, t3201: F, t398: F, t1090: F, t1181: F, t540: F, t1162: F, t2450: F, t1084: F, t4417: F, t1111: F, t1165: F) -> (F, F, F, F, F) {
    let t4728 = t398 * t3201 * t1449;
    let t4732 = t1181 * t540 * t1090;
    let t4735 = t2450 * t1162;
    let t4737 = t1181 * t4417 * t1084;
    let t4741 = t1165 * t4417 * t1111;
    (t4728, t4732, t4735, t4737, t4741)
}
