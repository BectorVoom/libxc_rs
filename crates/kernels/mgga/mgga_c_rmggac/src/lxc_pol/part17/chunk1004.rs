//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1004/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1004<F: Float>(t262: F, t46237: F, t35810: F, t352: F, t9884: F, t35815: F, t321: F, t46480: F, t46483: F, t46486: F, t46488: F, t46492: F, t46494: F, t46503: F, t46505: F, t46507: F, t46509: F, t5148: F, t8940: F) -> (F, F, F) {
    let t46511 = t262 * t46237;
    let t46512 = t35810 * t46511;
    let t46515 = t262 * t9884 * t352;
    let t46516 = t35815 * t46515;
    let t46518 = F::cast_from(0.40911992481368012592e-1_f64) * t46480 - F::cast_from(0.81823984962736025184e-1_f64) * t46483 - F::cast_from(0.40911992481368012592e-1_f64) * t46486 - F::cast_from(0.36366215538993788971e-1_f64) * t46488 - F::cast_from(0.90915538847484472429e-2_f64) * t46492 + F::cast_from(0.11974241701863808564e0_f64) * t8940 * t46494 * t352 - F::cast_from(0.11974241701863808564e0_f64) * t5148 * t46494 * t321 + F::cast_from(0.20455996240684006296e-1_f64) * t46503 - F::cast_from(0.40911992481368012592e-1_f64) * t46505 + F::cast_from(0.81823984962736025184e-1_f64) * t46507 - F::cast_from(0.13637330827122670864e-1_f64) * t46509 + F::cast_from(0.81823984962736025184e-1_f64) * t46512 + F::cast_from(0.20455996240684006296e-1_f64) * t46516;
    (t46511, t46515, t46518)
}
