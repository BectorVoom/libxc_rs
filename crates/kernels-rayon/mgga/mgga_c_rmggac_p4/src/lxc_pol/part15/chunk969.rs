//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 969/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk969(t2024: f64, t6463: f64, t1763: f64, t36288: f64, t1737: f64, t2064: f64, t3814: f64, t36292: f64, t305: f64, t2067: f64, t30526: f64, t9885: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46047 = t2024 * t6463;
    let t46050 = t36288 * t1763;
    let t46055 = t2064 * t1737;
    let t46056 = t3814 * t46055;
    let t46058 = t36292 * t1763;
    let t46059 = t305 * t46058;
    let t46062 = t30526 * t2067 * t9885;
    (t46047, t46050, t46055, t46056, t46058, t46059, t46062)
}
