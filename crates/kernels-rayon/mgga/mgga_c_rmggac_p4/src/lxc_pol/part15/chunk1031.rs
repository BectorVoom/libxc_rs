//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1031/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1031(t333: f64, t511: f64, t7230: f64, t7231: f64, t9988: f64, t352: f64, t515: f64, t118: f64, t2001: f64, t2281: f64, t615: f64, t7717: f64) -> (f64, f64, f64) {
    let t46953 = t7230 * t7231 * t511 * t9988 * t333;
    let t46958 = t7230 * t7231 * t515 * t9988 * t352;
    let t46962 = t2001 * t118 * t2281 * t615;
    let t46963 = t7717 * t46962;
    (t46953, t46958, t46963)
}
