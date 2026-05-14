//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 894/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk894<F: Float>(t2134: F, t27: F, t5840: F, t649: F, t46412: F, t8630: F, t46416: F, t7192: F, t2333: F, t39953: F, t7487: F, t9720: F, t35608: F, t35612: F, t35617: F, t35619: F, t35622: F, t35625: F, t35629: F, t35633: F, t40198: F, t40201: F, t40251: F, t46547: F, t739: F) -> (F,) {
    let t46811 = t2134 * t27 * t649 * t5840;
    let t46815 = t8630 * t46412;
    let t46817 = t7192 * t46416;
    let t46819 = t39953 * t2333;
    let t46821 = t7487 * t9720;
    let t46828 = 0.10227998120342003148e-1 * t46811 - 0.59871208509319042821e-1 * t739 * t46547 + 0.6818665413561335432e-1 * t46815 + 0.13637330827122670864e-1 * t46817 - 0.68186654135613354322e-2 * t46819 + 0.96056421943322389208e-3 * t46821 - 0.86737941314158990623e-4 * t40198 + 0.16260079888840015101e-2 * t40201 + t35608 - t35612 + t35617 - t35619 + t35622 + 0.36021158228745895953e-3 * t35625 + 0.30487649791575028314e-3 * t35629 + 0.30487649791575028314e-3 * t35633 - t40251;
    (t46828,)
}
