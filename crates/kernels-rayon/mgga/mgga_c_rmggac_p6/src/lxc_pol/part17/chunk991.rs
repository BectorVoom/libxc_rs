//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 991/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk991(t41483: f64, t570: f64, t36250: f64, t45569: f64, t35879: f64, t45573: f64, t36254: f64, t45578: f64, t1632: f64, t8975: f64, t1635: f64, t5898: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46324 = t41483 * t570;
    let t46327 = t36250 * t45569;
    let t46329 = t35879 * t45573;
    let t46331 = t36254 * t45578;
    let t46333 = t8975 * t1632;
    let t46336 = t8975 * t1635;
    let t46339 = t8975 * t5898;
    (t46324, t46327, t46329, t46331, t46333, t46336, t46339)
}
