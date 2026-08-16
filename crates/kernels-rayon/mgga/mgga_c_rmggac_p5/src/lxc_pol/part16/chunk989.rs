//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 989/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk989(t6444: f64, t9765: f64, t5840: f64, t645: f64, t793: f64, t46453: f64, t4669: f64, t44736: f64, t5259: f64, t46181: f64, t7844: f64, t1763: f64, t262: f64, t265: f64, t7835: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46609 = t6444 * t9765;
    let t46611 = t645 * t5840;
    let t46612 = t793 * t46611;
    let t46614 = t4669 * t46453;
    let t46634 = t5259 * t44736;
    let t46642 = t7844 * t46181;
    let t46646 = t7835 * t262 * t265 * t1763;
    (t46609, t46611, t46612, t46614, t46634, t46642, t46646)
}
