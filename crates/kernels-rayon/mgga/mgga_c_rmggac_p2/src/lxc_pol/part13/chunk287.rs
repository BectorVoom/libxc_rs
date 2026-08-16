//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 287/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk287(t1412: f64, t183: f64, t155: f64, t421: f64, t577: f64, t381: f64, t578: f64, t385: f64, t1020: f64, t1031: f64, t1011: f64, t1014: f64, t1027: f64, t1029: f64, t1044: f64, t1374: f64, t1392: f64, t436: f64, t948: f64, t975: f64, t982: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1413 = t1412 * t183;
    let t1414 = t155 * t1413;
    let t1415 = t577 * t421;
    let t1416 = t155 * t1415;
    let t1417 = t381 * t578;
    let t1418 = 4.0_f64 * t1417;
    let t1419 = t385 * t578;
    let t1420 = 4.0_f64 * t1419;
    let t1421 = 4.0_f64 * t1020;
    let t1422 = 4.0_f64 * t1031;
    let t1423 = t948 - t975 - t1374 + 0.93273e-1_f64 * t436 * t1392 + t1414 + t1416 + t1418 - t1420 + t982 + t1011 - t1014 - t1421 - t1027 + t1029 - t1422 - t1044;
    (t1413, t1414, t1415, t1416, t1418, t1420, t1421, t1422, t1423)
}
