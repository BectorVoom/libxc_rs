//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 998/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk998<F: Float>(t1550: F, t46611: F, t10102: F, t34884: F, t1652: F, t570: F, t1971: F, t3351: F, t875: F, t10040: F, t7720: F, t2310: F, t38351: F) -> (F, F, F, F, F, F) {
    let t46863 = t1550 * t46611;
    let t46865 = t34884 * t10102;
    let t46867 = t570 * t1652;
    let t46870 = t3351 * t1971 * t875 * t46867;
    let t46873 = t7720 * t10040;
    let t46875 = t38351 * t2310;
    (t46863, t46865, t46867, t46870, t46873, t46875)
}
