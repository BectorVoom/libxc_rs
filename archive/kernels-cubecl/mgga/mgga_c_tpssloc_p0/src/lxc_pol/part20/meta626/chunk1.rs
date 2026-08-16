//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2257/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2257<F: Float>(t212: F, t4119: F, t2586: F, t9523: F, t4138: F, t9541: F, t41189: F, t4134: F, t118: F, t12971: F, t2576: F, t794: F) -> (F, F, F, F, F) {
    let t46766 = t212 * t4119;
    let t46768 = t2586 * t9523 * t46766;
    let t46769 = F::cast_from(0.49999999999999999998e-2_f64) * t46768;
    let t46770 = t9541 * t4138;
    let t46772 = t41189 * t4134;
    let t46780 = t2576 * t118 * t794 * t12971;
    (t46766, t46769, t46770, t46772, t46780)
}
