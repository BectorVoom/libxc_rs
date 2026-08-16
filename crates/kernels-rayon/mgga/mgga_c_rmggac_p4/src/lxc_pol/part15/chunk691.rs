//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 691/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk691(t1707: f64, t649: f64, t7599: f64, t7603: f64, t117: f64, t1704: f64) -> (f64, f64, f64) {
    let t9903 = t649 * t1707;
    let t9904 = t7599 * t9903;
    let t9906 = t7603 * t9903;
    let t9908 = t1704 * t117;
    (t9904, t9906, t9908)
}
