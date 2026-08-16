//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1082/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1082(t4601: f64, t9739: f64, t2060: f64, t30283: f64, t903: f64, t30360: f64, t46502: f64, t7204: f64, t46358: f64, t36797: f64, t36802: f64, t36804: f64, t36809: f64, t41960: f64, t46375: f64, t47719: f64, t47721: f64, t47723: f64, t47725: f64, t47727: f64, t47729: f64, t47735: f64, t884: f64) -> f64 {
    let t47737 = t4601 * t9739;
    let t47740 = t903 * t2060 * t30283;
    let t47743 = t903 * t2060 * t30360;
    let t47745 = t7204 * t46502;
    let t47747 = t7204 * t46358;
    let t47749 = 0.11974241701863808564e0_f64 * t884 * t46375 - 0.23948483403727617128e0_f64 * t47719 + 0.27274661654245341728e-1_f64 * t47721 + 0.27274661654245341728e-1_f64 * t47723 + 0.20455996240684006297e-1_f64 * t47725 - 0.27274661654245341729e-1_f64 * t47727 - 0.20455996240684006297e-1_f64 * t47729 - t36797 + t36802 + 0.81300399444200075504e-3_f64 * t36804 + 0.81300399444200075504e-3_f64 * t36809 - 0.59590439850616975157e-4_f64 * t41960 - 0.35922725105591425692e0_f64 * t47735 - 0.8980681276397856423e-1_f64 * t47737 - 0.8980681276397856423e-1_f64 * t47740 - 0.8980681276397856423e-1_f64 * t47743 - 0.20455996240684006296e-1_f64 * t47745 - 0.20455996240684006296e-1_f64 * t47747;
    t47749
}
