//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 870/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk870<F: Float>(t30248: F, t425: F, t1020: F, t7614: F, t1029: F, t7605: F, t7478: F, t7637: F, t1160: F, t7584: F, t1992: F, t4210: F, t7842: F) -> (F, F, F, F, F, F) {
    let t30249 = t30248 * t425;
    let t30251 = t7614 * t1020;
    let t30253 = t7605 * t1029;
    let t30260 = t7637 * t7478;
    let t30262 = t1160 * t7584;
    let t30265 = t30262 * t7842 * t1992 * t4210;
    (t30249, t30251, t30253, t30260, t30262, t30265)
}
