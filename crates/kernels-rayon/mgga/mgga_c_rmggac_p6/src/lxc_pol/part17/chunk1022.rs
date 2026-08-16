//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1022/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1022(t35608: f64, t35612: f64, t35617: f64, t35619: f64, t35622: f64, t35625: f64, t35629: f64, t35633: f64, t40198: f64, t40201: f64, t40251: f64, t46547: f64, t46811: f64, t46815: f64, t46817: f64, t46819: f64, t46821: f64, t739: f64) -> f64 {
    let t46828 = 0.10227998120342003148e-1_f64 * t46811 - 0.59871208509319042821e-1_f64 * t739 * t46547 + 0.6818665413561335432e-1_f64 * t46815 + 0.13637330827122670864e-1_f64 * t46817 - 0.68186654135613354322e-2_f64 * t46819 + 0.96056421943322389208e-3_f64 * t46821 - 0.86737941314158990623e-4_f64 * t40198 + 0.16260079888840015101e-2_f64 * t40201 + t35608 - t35612 + t35617 - t35619 + t35622 + 0.36021158228745895953e-3_f64 * t35625 + 0.30487649791575028314e-3_f64 * t35629 + 0.30487649791575028314e-3_f64 * t35633 - t40251;
    t46828
}
