//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 988/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk988<F: Float>(t262: F, t46541: F, t35929: F, t46261: F, t5271: F, t46437: F, t5259: F, t45166: F, t5148: F, t44732: F, t46419: F, t46423: F, t4669: F) -> (F, F, F, F, F, F, F, F) {
    let t46542 = t262 * t46541;
    let t46543 = t35929 * t46542;
    let t46554 = t5271 * t46261;
    let t46556 = t5259 * t46437;
    let t46599 = t5148 * t45166;
    let t46603 = t5271 * t44732;
    let t46605 = t5259 * t46419;
    let t46607 = t4669 * t46423;
    (t46542, t46543, t46554, t46556, t46599, t46603, t46605, t46607)
}
