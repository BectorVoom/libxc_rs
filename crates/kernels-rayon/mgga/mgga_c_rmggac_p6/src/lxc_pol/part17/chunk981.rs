//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 981/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk981(t1707: f64, t2084: f64, t7599: f64, t7603: f64, t46164: f64, t8764: f64, t46167: f64, t3826: f64, t44732: f64, t3851: f64, t3839: f64, t45720: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46211 = t2084 * t1707;
    let t46212 = t7599 * t46211;
    let t46214 = t7603 * t46211;
    let t46216 = t8764 * t46164;
    let t46218 = t7599 * t46167;
    let t46220 = t3826 * t44732;
    let t46222 = t3851 * t44732;
    let t46224 = t3839 * t45720;
    (t46212, t46214, t46216, t46218, t46220, t46222, t46224)
}
