//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3218/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3218<F: Float>(t1851: F, t5381: F, t20148: F, t580: F, t20186: F, t576: F, t1395: F, t6483: F, t1404: F, t6470: F, t1858: F, t5363: F) -> (F, F, F, F, F, F) {
    let t66964 = t1851 * t5381;
    let t66967 = t20148 * t580;
    let t66976 = t576 * t20186;
    let t66987 = t1395 * t6483;
    let t66989 = t6470 * t1404;
    let t66991 = t5363 * t1858;
    (t66964, t66967, t66976, t66987, t66989, t66991)
}
