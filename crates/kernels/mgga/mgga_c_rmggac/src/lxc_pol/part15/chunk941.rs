//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 941/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk941<F: Float>(t2060: F, t30283: F, t903: F, t30360: F, t46502: F, t7204: F, t46358: F, t36797: F, t36802: F, t36804: F, t36809: F, t41960: F, t46375: F, t47719: F, t47721: F, t47723: F, t47725: F, t47727: F, t47729: F, t47735: F, t47737: F, t884: F) -> (F,) {
    let t47740 = t903 * t2060 * t30283;
    let t47743 = t903 * t2060 * t30360;
    let t47745 = t7204 * t46502;
    let t47747 = t7204 * t46358;
    let t47749 = 0.11974241701863808564e0 * t884 * t46375 - 0.23948483403727617128e0 * t47719 + 0.27274661654245341728e-1 * t47721 + 0.27274661654245341728e-1 * t47723 + 0.20455996240684006297e-1 * t47725 - 0.27274661654245341729e-1 * t47727 - 0.20455996240684006297e-1 * t47729 - t36797 + t36802 + 0.81300399444200075504e-3 * t36804 + 0.81300399444200075504e-3 * t36809 - 0.59590439850616975157e-4 * t41960 - 0.35922725105591425692e0 * t47735 - 0.8980681276397856423e-1 * t47737 - 0.8980681276397856423e-1 * t47740 - 0.8980681276397856423e-1 * t47743 - 0.20455996240684006296e-1 * t47745 - 0.20455996240684006296e-1 * t47747;
    (t47749,)
}
