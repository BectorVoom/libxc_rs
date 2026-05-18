//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1022/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1022<F: Float>(t35608: F, t35612: F, t35617: F, t35619: F, t35622: F, t35625: F, t35629: F, t35633: F, t40198: F, t40201: F, t40251: F, t46547: F, t46811: F, t46815: F, t46817: F, t46819: F, t46821: F, t739: F) -> F {
    let t46828 = F::new(0.10227998120342003148e-1) * t46811 - F::new(0.59871208509319042821e-1) * t739 * t46547 + F::new(0.6818665413561335432e-1) * t46815 + F::new(0.13637330827122670864e-1) * t46817 - F::new(0.68186654135613354322e-2) * t46819 + F::new(0.96056421943322389208e-3) * t46821 - F::new(0.86737941314158990623e-4) * t40198 + F::new(0.16260079888840015101e-2) * t40201 + t35608 - t35612 + t35617 - t35619 + t35622 + F::new(0.36021158228745895953e-3) * t35625 + F::new(0.30487649791575028314e-3) * t35629 + F::new(0.30487649791575028314e-3) * t35633 - t40251;
    t46828
}
