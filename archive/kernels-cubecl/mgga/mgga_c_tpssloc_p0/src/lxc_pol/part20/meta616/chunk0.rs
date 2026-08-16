//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2224/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2224<F: Float>(t46137: F, t40667: F, t40670: F, t40673: F, t40680: F, t40682: F, t39309: F, t39312: F, t39316: F, t39320: F, t40679: F, t40685: F) -> (F, F, F, F, F, F, F) {
    let t46138 = F::cast_from(0.32530743900905219526e-1_f64) * t46137;
    let t46140 = F::cast_from(0.15584273195113317383e3_f64) * t40667;
    let t46141 = F::cast_from(0.18311447306006545054e-3_f64) * t40670;
    let t46142 = F::cast_from(3.0_f64) * t40673;
    let t46143 = F::cast_from(0.73245789224026180215e-3_f64) * t40680;
    let t46144 = F::cast_from(0.10526802520742363173e2_f64) * t40682;
    let t46145 = -t39309 + t39312 + t39316 + t39320 - t46140 - t46141 + t46142 - t40679 + t46143 + t46144 - t40685;
    (t46138, t46140, t46141, t46142, t46143, t46144, t46145)
}
