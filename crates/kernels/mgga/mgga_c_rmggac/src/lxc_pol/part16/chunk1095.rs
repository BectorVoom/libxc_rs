//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1095/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1095<F: Float>(t35625: F, t35629: F, t35633: F, t37786: F, t37787: F, t37788: F, t37789: F, t37790: F, t40198: F, t40201: F, t46806: F, t46811: F, t46815: F, t46817: F, t46819: F, t46821: F, t4985: F, t9370: F) -> F {
    let t48777 = F::cast_from(0.17961362552795712846e0_f64) * t46806 + F::cast_from(0.20455996240684006298e-1_f64) * t46811 + F::cast_from(0.11974241701863808564e0_f64) * t4985 * t9370 + F::cast_from(0.13637330827122670865e0_f64) * t46815 + F::cast_from(0.2727466165424534173e-1_f64) * t46817 - F::cast_from(0.13637330827122670865e-1_f64) * t46819 + F::cast_from(0.1921128438866447784e-2_f64) * t46821 - F::cast_from(0.17347588262831798123e-3_f64) * t40198 + F::cast_from(0.325201597776800302e-2_f64) * t40201 + t37786 - t37787 + t37788 - t37789 + t37790 + F::cast_from(0.72042316457491791901e-3_f64) * t35625 + F::cast_from(0.60975299583150056624e-3_f64) * t35629 + F::cast_from(0.60975299583150056624e-3_f64) * t35633;
    t48777
}
