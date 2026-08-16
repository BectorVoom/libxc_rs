//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2233/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2233<F: Float>(t40748: F, t40760: F, t40764: F, t40766: F, t46282: F, t46284: F, t46286: F, t46287: F, t46288: F, t46292: F, t46293: F, t2379: F, t868: F) -> (F, F) {
    let t46294 = t46282 + t46284 + t46286 + t40748 + t46287 + t40760 - t46288 + t46292 + t40764 + t40766 + t46293;
    let t46298 = t2379 * t868;
    (t46294, t46298)
}
