//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 971/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk971(t46116: f64, t851: f64, t46121: f64, t797: f64, t46128: f64, t793: f64, t1707: f64, t2084: f64, t7599: f64, t7603: f64, t46164: f64, t8764: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46197 = t851 * t46116;
    let t46199 = t797 * t46121;
    let t46201 = t793 * t46128;
    let t46211 = t2084 * t1707;
    let t46212 = t7599 * t46211;
    let t46214 = t7603 * t46211;
    let t46216 = t8764 * t46164;
    (t46197, t46199, t46201, t46212, t46214, t46216)
}
