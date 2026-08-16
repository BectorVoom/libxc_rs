//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 985/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk985(t2073: f64, t46177: f64, t1756: f64, t2079: f64, t262: f64, t265: f64, t570: f64, t8704: f64, t7788: f64, t45721: f64, t7844: f64, t45727: f64, t7785: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46488 = t2073 * t46177;
    let t46492 = t2079 * t262 * t265 * t1756;
    let t46501 = t8704 * t570;
    let t46502 = t262 * t46501;
    let t46503 = t7788 * t46502;
    let t46505 = t7844 * t45721;
    let t46507 = t7785 * t45727;
    (t46488, t46492, t46501, t46502, t46503, t46505, t46507)
}
