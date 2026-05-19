//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1022/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1022<F: Float>(t35608: F, t35612: F, t35617: F, t35619: F, t35622: F, t35625: F, t35629: F, t35633: F, t40198: F, t40201: F, t40251: F, t46547: F, t46811: F, t46815: F, t46817: F, t46819: F, t46821: F, t739: F) -> F {
    let t46828 = F::cast_from(0.10227998120342003148e-1_f64) * t46811 - F::cast_from(0.59871208509319042821e-1_f64) * t739 * t46547 + F::cast_from(0.6818665413561335432e-1_f64) * t46815 + F::cast_from(0.13637330827122670864e-1_f64) * t46817 - F::cast_from(0.68186654135613354322e-2_f64) * t46819 + F::cast_from(0.96056421943322389208e-3_f64) * t46821 - F::cast_from(0.86737941314158990623e-4_f64) * t40198 + F::cast_from(0.16260079888840015101e-2_f64) * t40201 + t35608 - t35612 + t35617 - t35619 + t35622 + F::cast_from(0.36021158228745895953e-3_f64) * t35625 + F::cast_from(0.30487649791575028314e-3_f64) * t35629 + F::cast_from(0.30487649791575028314e-3_f64) * t35633 - t40251;
    t46828
}
