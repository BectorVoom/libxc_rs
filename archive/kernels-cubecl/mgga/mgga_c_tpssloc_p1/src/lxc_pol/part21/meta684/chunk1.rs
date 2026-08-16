//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2498/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2498<F: Float>(t12994: F, t13012: F, t213: F, t221: F, t13004: F, t782: F, t13007: F, t131: F, t205: F, t41160: F, t116: F, t212: F) -> (F, F, F, F, F, F) {
    let t46836 = t13012 * t12994;
    let t46838 = t221 * t213;
    let t46843 = t782 * t13004;
    let t46844 = t46843 * t13007;
    let t46847 = t205 * t41160 * t131;
    let t46853 = t116 * t212;
    (t46836, t46838, t46843, t46844, t46847, t46853)
}
