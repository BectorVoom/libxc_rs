//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 994/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk994<F: Float>(t46415: F, t4669: F, t27048: F, t46541: F, t46525: F, t1550: F, t30800: F, t7577: F, t30490: F, t903: F, t35972: F, t45556: F, t739: F) -> (F, F, F, F, F, F) {
    let t46774 = t4669 * t46415;
    let t46782 = t27048 * t46541;
    let t46784 = t4669 * t46525;
    let t46800 = t1550 * t7577 * t30800;
    let t46803 = t903 * t7577 * t30490;
    let t46806 = t739 * t35972 * t45556;
    (t46774, t46782, t46784, t46800, t46803, t46806)
}
