//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 978/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk978(t36250: f64, t45569: f64, t35879: f64, t45573: f64, t36254: f64, t45578: f64, t35960: f64, t649: f64, t6583: f64, t41400: f64, t6586: f64, t40932: f64, t6558: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46327 = t36250 * t45569;
    let t46329 = t35879 * t45573;
    let t46331 = t36254 * t45578;
    let t46343 = t35960 * t649 * t6583;
    let t46346 = t41400 * t649 * t6586;
    let t46349 = t40932 * t649 * t6558;
    (t46327, t46329, t46331, t46343, t46346, t46349)
}
