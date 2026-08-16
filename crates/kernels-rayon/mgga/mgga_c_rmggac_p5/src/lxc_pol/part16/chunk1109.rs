//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1109/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1109(t43507: f64, t43513: f64, t43518: f64, t46084: f64, t46087: f64, t46090: f64, t46093: f64, t46096: f64, t46099: f64, t46102: f64, t46107: f64, t46110: f64, t46112: f64, t46114: f64, t46118: f64) -> f64 {
    let t49048 = -0.5454932330849068346e-1_f64 * t46084 + 0.2727466165424534173e-1_f64 * t46087 - 0.5454932330849068346e-1_f64 * t46090 - 0.5454932330849068346e-1_f64 * t46093 + 0.13637330827122670865e0_f64 * t46096 - 0.2727466165424534173e0_f64 * t46099 + 0.90915538847484472432e-1_f64 * t46102 - t43507 + t43513 - t43518 - 0.10909864661698136692e0_f64 * t46107 + 0.2727466165424534173e0_f64 * t46110 - 0.12700854093841289481e-1_f64 * t46112 + 0.2032136655014606317e-1_f64 * t46114 + 0.1814407727691612783e-3_f64 * t46118;
    t49048
}
