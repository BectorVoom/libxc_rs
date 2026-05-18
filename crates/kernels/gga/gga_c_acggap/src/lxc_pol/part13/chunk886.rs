//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 886/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk886<F: Float>(t1165: F, t3759: F, t7351: F, t7426: F, t3360: F, t7646: F, t3393: F, t7361: F, t7433: F, t7353: F, t1181: F, t16548: F, t599: F, t7346: F) -> (F, F, F, F, F, F) {
    let t30463 = t7426 * t1165 * t7351 * t3759;
    let t30468 = t3360 * t7646;
    let t30469 = t30468 * t3393;
    let t30497 = t7433 * t7361;
    let t30507 = t7433 * t7353;
    let t30511 = t7346 * t1181 * t599 * t16548;
    (t30463, t30468, t30469, t30497, t30507, t30511)
}
