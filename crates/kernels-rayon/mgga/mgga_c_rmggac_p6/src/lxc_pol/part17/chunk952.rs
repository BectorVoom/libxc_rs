//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 952/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk952(t10195: f64, t275: f64, t30400: f64, t3351: f64, t3352: f64, t515: f64, t2286: f64, t38351: f64, t38530: f64, t9171: f64, t14243: f64, t16503: f64, t552: f64, t9157: f64) -> (f64, f64, f64, f64, f64) {
    let t45798 = t275 * t10195;
    let t45811 = t3351 * t3352 * t515 * t30400;
    let t45813 = t38351 * t2286;
    let t45818 = t38530 * t9171;
    let t45822 = t16503 * t14243 * t552 * t9157;
    (t45798, t45811, t45813, t45818, t45822)
}
