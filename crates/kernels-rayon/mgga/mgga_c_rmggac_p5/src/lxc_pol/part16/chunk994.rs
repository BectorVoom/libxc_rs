//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 994/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk994(t46415: f64, t4669: f64, t27048: f64, t46541: f64, t46525: f64, t1550: f64, t30800: f64, t7577: f64, t30490: f64, t903: f64, t35972: f64, t45556: f64, t739: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46774 = t4669 * t46415;
    let t46782 = t27048 * t46541;
    let t46784 = t4669 * t46525;
    let t46800 = t1550 * t7577 * t30800;
    let t46803 = t903 * t7577 * t30490;
    let t46806 = t739 * t35972 * t45556;
    (t46774, t46782, t46784, t46800, t46803, t46806)
}
