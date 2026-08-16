//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 969/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk969(t46164: f64, t8750: f64, t6418: f64, t649: f64, t7603: f64, t46139: f64, t46142: f64, t8761: f64, t1743: f64, t265: f64, t262: f64, t2103: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t46165 = t8750 * t46164;
    let t46167 = t649 * t6418;
    let t46168 = t7603 * t46167;
    let t46170 = t7603 * t46139;
    let t46172 = t8761 * t46142;
    let t46176 = t265 * t1743;
    let t46177 = t262 * t46176;
    let t46178 = t2103 * t46177;
    (t46165, t46167, t46168, t46170, t46172, t46176, t46177, t46178)
}
