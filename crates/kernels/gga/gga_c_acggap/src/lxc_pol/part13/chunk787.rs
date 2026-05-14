//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 787/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk787<F: Float>(t1111: F, t30147: F, t30148: F, t7842: F, t7335: F, t7583: F, t2450: F) -> (F, F, F) {
    let t30151 = t30147 * t7842 * t30148 * t1111;
    let t30153 = t7583 * t7335;
    let t30154 = t2450 * t30153;
    (t30151, t30153, t30154)
}
