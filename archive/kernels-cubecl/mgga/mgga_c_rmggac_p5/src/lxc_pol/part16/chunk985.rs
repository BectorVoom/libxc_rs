//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 985/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk985<F: Float>(t2073: F, t46177: F, t1756: F, t2079: F, t262: F, t265: F, t570: F, t8704: F, t7788: F, t45721: F, t7844: F, t45727: F, t7785: F) -> (F, F, F, F, F, F, F) {
    let t46488 = t2073 * t46177;
    let t46492 = t2079 * t262 * t265 * t1756;
    let t46501 = t8704 * t570;
    let t46502 = t262 * t46501;
    let t46503 = t7788 * t46502;
    let t46505 = t7844 * t45721;
    let t46507 = t7785 * t45727;
    (t46488, t46492, t46501, t46502, t46503, t46505, t46507)
}
