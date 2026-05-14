//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 870/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk870<F: Float>(t41483: F, t570: F, t36250: F, t45569: F, t35879: F, t45573: F, t36254: F, t45578: F, t1632: F, t8975: F, t1635: F, t5898: F, t35960: F, t649: F, t6583: F, t41400: F, t6586: F) -> (F, F, F, F, F, F, F, F, F) {
    let t46324 = t41483 * t570;
    let t46327 = t36250 * t45569;
    let t46329 = t35879 * t45573;
    let t46331 = t36254 * t45578;
    let t46333 = t8975 * t1632;
    let t46336 = t8975 * t1635;
    let t46339 = t8975 * t5898;
    let t46343 = t35960 * t649 * t6583;
    let t46346 = t41400 * t649 * t6586;
    (t46324, t46327, t46329, t46331, t46333, t46336, t46339, t46343, t46346)
}
