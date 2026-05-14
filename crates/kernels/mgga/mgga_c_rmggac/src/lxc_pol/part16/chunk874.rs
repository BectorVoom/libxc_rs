//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 874/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk874<F: Float>(t36629: F, t6387: F, t656: F, t36471: F, t6530: F, t2604: F, t9812: F, t1550: F, t46611: F, t10102: F, t34884: F, t1652: F, t570: F, t1971: F, t3351: F, t875: F) -> (F, F, F, F, F, F, F) {
    let t46856 = t36629 * t656 * t6387;
    let t46859 = t36471 * t656 * t6530;
    let t46861 = t2604 * t9812;
    let t46863 = t1550 * t46611;
    let t46865 = t34884 * t10102;
    let t46867 = t570 * t1652;
    let t46870 = t3351 * t1971 * t875 * t46867;
    (t46856, t46859, t46861, t46863, t46865, t46867, t46870)
}
