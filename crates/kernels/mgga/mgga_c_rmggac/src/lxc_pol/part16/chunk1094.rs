//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1094/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1094<F: Float>(t1664: F, t2474: F, t289: F, t40123: F, t40125: F, t40127: F, t40128: F, t40129: F, t43288: F, t46022: F, t46024: F, t46026: F, t46034: F, t46038: F, t46040: F, t46043: F, t46045: F, t46800: F, t46803: F) -> F {
    let t48753 = t1664 * t2474;
    let t48763 = -F::cast_from(0.5107751987195740728e-4_f64) * t46022 + F::cast_from(0.212822999466489197e-4_f64) * t46024 + F::cast_from(0.11918087970123395032e-3_f64) * t46026 - F::cast_from(0.5107751987195740728e-4_f64) * t46034 - F::cast_from(0.5107751987195740728e-4_f64) * t46038 - F::new(0.4726e1) * t289 * t48753 - F::cast_from(0.11974241701863808564e0_f64) * t46040 + F::cast_from(0.17961362552795712846e0_f64) * t46043 + F::cast_from(0.5987120850931904282e-1_f64) * t46045 + t43288 + F::cast_from(0.49658699875514145965e-4_f64) * t40123 + F::cast_from(0.49658699875514145965e-4_f64) * t40125 - t40127 + t40128 + t40129 - F::cast_from(0.11974241701863808564e0_f64) * t46800 + F::cast_from(0.17961362552795712846e0_f64) * t46803;
    t48763
}
