//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 264/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk264(t1046: f64, t195: f64, t618: f64, t1412: f64, t181: f64, t446: f64, t589: f64, t1011: f64, t1014: f64, t1027: f64, t1029: f64, t1374: f64, t1414: f64, t1416: f64, t1418: f64, t1420: f64, t1421: f64, t948: f64, t975: f64, t982: f64) -> (f64, f64, f64, f64, f64) {
    let t1424 = 0.5848223622634646207e0_f64 * t1046;
    let t1425 = t195 * t618;
    let t1429 = 0.19751673498613801407e-1_f64 * t1412 * t181;
    let t1430 = t589 * t446;
    let t1433 = t948 - t975 - t1374 + t1414 + t1416 + t1418 - t1420 + t982 + t1011 - t1014 - t1421 - t1027 + t1029;
    (t1424, t1425, t1429, t1430, t1433)
}
