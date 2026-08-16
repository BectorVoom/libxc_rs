//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 421/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk421(t1413: f64, t1427: f64, t1429: f64, t1437: f64, t1442: f64, t1449: f64, t294: f64, t305: f64, t877: f64, t896: f64, t1448: f64, t895: f64, t904: f64) -> (f64, f64, f64) {
    let t1453 = t294 * (-0.310907e-1_f64 * t1429 * t305 + 1.0_f64 * t877 * t1437 + t1413 - t1427 - 0.19751673498613801407e-1_f64 * t1442 + 0.5848223622634646207e0_f64 * t896 * t1449);
    let t1455 = 0.19751673498613801407e-1_f64 * t294 * t1442;
    let t1457 = t895 * t1448 * t904;
    (t1453, t1455, t1457)
}
