//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 917/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk917(t30204: f64, t76320: f64, t14125: f64, t14131: f64, t8436: f64, t15252: f64, t3351: f64, t498: f64, t515: f64, t9210: f64, t3928: f64, t76270: f64) -> (f64, f64, f64, f64) {
    let t76497 = 0.11974241701863808564e0_f64 * t30204 * t76320;
    let t76499 = t14131 * t14125 * t8436;
    let t76504 = t3351 * t9210 * t515 * t15252 * t498;
    let t76506 = t3928 * t76270;
    (t76497, t76499, t76504, t76506)
}
