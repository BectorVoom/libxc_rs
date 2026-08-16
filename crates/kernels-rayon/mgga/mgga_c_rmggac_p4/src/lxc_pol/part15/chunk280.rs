//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 280/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk280(t302: f64, t574: f64, t1551: f64, t793: f64, t1554: f64, t797: f64, t338: f64, t551: f64) -> (f64, f64, f64, f64) {
    let t1591 = t302 * t574;
    let t1594 = t793 * t1551;
    let t1596 = t797 * t1554;
    let t1598 = t338 * t551;
    (t1591, t1594, t1596, t1598)
}
