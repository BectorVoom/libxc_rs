//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 301/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk301(t302: f64, t574: f64, t1551: f64, t793: f64, t1554: f64, t797: f64, t338: f64, t551: f64, t352: f64, t305: f64, t128: f64, t1587: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1591 = t302 * t574;
    let t1594 = t793 * t1551;
    let t1596 = t797 * t1554;
    let t1598 = t338 * t551;
    let t1599 = t1598 * t352;
    let t1600 = t305 * t1599;
    let t1602 = t128 * t1587;
    (t1591, t1594, t1596, t1598, t1600, t1602)
}
