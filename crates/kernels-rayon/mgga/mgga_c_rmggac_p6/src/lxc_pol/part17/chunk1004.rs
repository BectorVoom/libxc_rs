//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1004/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1004(t262: f64, t46237: f64, t35810: f64, t352: f64, t9884: f64, t35815: f64, t321: f64, t46480: f64, t46483: f64, t46486: f64, t46488: f64, t46492: f64, t46494: f64, t46503: f64, t46505: f64, t46507: f64, t46509: f64, t5148: f64, t8940: f64) -> (f64, f64, f64) {
    let t46511 = t262 * t46237;
    let t46512 = t35810 * t46511;
    let t46515 = t262 * t9884 * t352;
    let t46516 = t35815 * t46515;
    let t46518 = 0.40911992481368012592e-1_f64 * t46480 - 0.81823984962736025184e-1_f64 * t46483 - 0.40911992481368012592e-1_f64 * t46486 - 0.36366215538993788971e-1_f64 * t46488 - 0.90915538847484472429e-2_f64 * t46492 + 0.11974241701863808564e0_f64 * t8940 * t46494 * t352 - 0.11974241701863808564e0_f64 * t5148 * t46494 * t321 + 0.20455996240684006296e-1_f64 * t46503 - 0.40911992481368012592e-1_f64 * t46505 + 0.81823984962736025184e-1_f64 * t46507 - 0.13637330827122670864e-1_f64 * t46509 + 0.81823984962736025184e-1_f64 * t46512 + 0.20455996240684006296e-1_f64 * t46516;
    (t46511, t46515, t46518)
}
