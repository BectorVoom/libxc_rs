//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1129/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1129<F: Float>(t326: F, t46471: F, t46473: F, t46476: F, t46480: F, t46483: F, t46486: F, t46488: F, t46492: F, t46503: F, t46505: F, t46507: F, t48482: F) -> F {
    let t49380 = F::cast_from(0.32729593985094410076e0_f64) * t46471 - F::cast_from(0.8182398496273602519e0_f64) * t46473 - F::cast_from(0.16364796992547205038e0_f64) * t46476 + F::cast_from(0.81823984962736025192e-1_f64) * t46480 - F::cast_from(0.16364796992547205038e0_f64) * t46483 - F::cast_from(0.81823984962736025192e-1_f64) * t46486 - F::cast_from(0.72732431077987577947e-1_f64) * t46488 - F::cast_from(0.18183107769496894487e-1_f64) * t46492 - F::cast_from(0.11974241701863808564e0_f64) * t326 * t48482 + F::cast_from(0.40911992481368012596e-1_f64) * t46503 - F::cast_from(0.81823984962736025192e-1_f64) * t46505 + F::cast_from(0.16364796992547205038e0_f64) * t46507;
    t49380
}
