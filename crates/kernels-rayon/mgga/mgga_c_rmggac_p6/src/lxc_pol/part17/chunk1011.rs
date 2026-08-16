//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1011/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1011(t44736: f64, t5259: f64, t46181: f64, t7844: f64, t1763: f64, t262: f64, t265: f64, t7835: f64, t46185: f64, t7829: f64, t2068: f64, t46117: f64) -> (f64, f64, f64, f64, f64) {
    let t46634 = t5259 * t44736;
    let t46642 = t7844 * t46181;
    let t46646 = t7835 * t262 * t265 * t1763;
    let t46648 = t7829 * t46185;
    let t46650 = t2068 * t46117;
    (t46634, t46642, t46646, t46648, t46650)
}
