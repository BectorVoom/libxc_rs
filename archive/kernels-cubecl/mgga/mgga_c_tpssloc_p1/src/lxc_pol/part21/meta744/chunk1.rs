//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2613/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2613<F: Float>(t11801: F, t5005: F, t15032: F, t3576: F, t11713: F, t11716: F, t53081: F, t11786: F, t5024: F, t3032: F, t52434: F, t3505: F) -> (F, F, F, F, F, F) {
    let t53291 = t5005 * t11801;
    let t53322 = t15032 * t3576;
    let t53336 = t11713 * t11716 * t53081;
    let t53360 = t5024 * t11786;
    let t53371 = t52434 * t3032;
    let t53372 = t53371 * t3505;
    (t53291, t53322, t53336, t53360, t53371, t53372)
}
