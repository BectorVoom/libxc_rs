//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1094/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1094(t1664: f64, t2474: f64, t289: f64, t40123: f64, t40125: f64, t40127: f64, t40128: f64, t40129: f64, t43288: f64, t46022: f64, t46024: f64, t46026: f64, t46034: f64, t46038: f64, t46040: f64, t46043: f64, t46045: f64, t46800: f64, t46803: f64) -> f64 {
    let t48753 = t1664 * t2474;
    let t48763 = -0.5107751987195740728e-4_f64 * t46022 + 0.212822999466489197e-4_f64 * t46024 + 0.11918087970123395032e-3_f64 * t46026 - 0.5107751987195740728e-4_f64 * t46034 - 0.5107751987195740728e-4_f64 * t46038 - 0.4726e1_f64 * t289 * t48753 - 0.11974241701863808564e0_f64 * t46040 + 0.17961362552795712846e0_f64 * t46043 + 0.5987120850931904282e-1_f64 * t46045 + t43288 + 0.49658699875514145965e-4_f64 * t40123 + 0.49658699875514145965e-4_f64 * t40125 - t40127 + t40128 + t40129 - 0.11974241701863808564e0_f64 * t46800 + 0.17961362552795712846e0_f64 * t46803;
    t48763
}
