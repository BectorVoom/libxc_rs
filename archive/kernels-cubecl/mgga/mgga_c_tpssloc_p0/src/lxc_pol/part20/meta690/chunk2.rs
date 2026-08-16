//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2620/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2620<F: Float>(t11697: F, t15469: F, t3577: F, t11801: F, t5005: F, t3247: F, t475: F, t15032: F, t3576: F, t11713: F, t11716: F, t53081: F) -> (F, F, F, F, F) {
    let t53287 = t3577 * t11697 * t15469;
    let t53291 = t5005 * t11801;
    let t53298 = t475 * t3247;
    let t53322 = t15032 * t3576;
    let t53336 = t11713 * t11716 * t53081;
    (t53287, t53291, t53298, t53322, t53336)
}
