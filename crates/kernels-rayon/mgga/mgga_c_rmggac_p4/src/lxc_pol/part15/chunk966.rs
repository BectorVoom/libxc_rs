//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 966/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk966(t1756: f64, t352: f64, t118: f64, t128: f64, t1986: f64, t1994: f64, t6258: f64, t2289: f64, t38355: f64, t8571: f64, t8592: f64, t34847: f64, t9845: f64) -> (f64, f64, f64, f64, f64) {
    let t46005 = t1756 * t352;
    let t46018 = t1994 * t1986 * t118 * t128 * t6258;
    let t46020 = t38355 * t2289;
    let t46022 = t8571 * t8592;
    let t46024 = t34847 * t9845;
    (t46005, t46018, t46020, t46022, t46024)
}
