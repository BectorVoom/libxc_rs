//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1261/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1261<F: Float>(t22960: F, t46298: F, t46252: F, t25373: F, t46362: F, t2249: F, t776: F, t2553: F, t606: F, t25: F, t9516: F, t868: F) -> (F, F, F, F, F, F, F) {
    let t81486 = t22960 * t46298;
    let t81489 = t22960 * t46252;
    let t81492 = t25373 * t46362;
    let t81501 = t2249 * t776;
    let t81505 = t606 * t2553;
    let t81509 = t25 * t9516;
    let t81513 = t2249 * t868;
    (t81486, t81489, t81492, t81501, t81505, t81509, t81513)
}
