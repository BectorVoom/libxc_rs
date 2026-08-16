//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 402/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk402(t45: f64, t57: f64, t1289: f64, t190: f64, t681: f64, t78: f64, t81: f64, t150: f64, t162: f64, t187: f64, t741: f64, t745: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t1342 = t190 * t1289;
    let t1344 = 4.0_f64 * t681 * t1342;
    let t1347 = piecewise3(t151, 0.0_f64, 4.0_f64 / 3.0_f64 * t78 * t1289);
    let t1350 = piecewise3(t155, 0.0_f64, -4.0_f64 / 3.0_f64 * t81 * t1289);
    let t1351 = t1347 + t1350;
    let t1352 = t150 * t1351;
    let t1353 = t1352 * t190;
    let t1354 = t1351 * t162;
    let t1356 = 0.19751673498613801407e-1_f64 * t1354 * t187;
    let t1359 = piecewise3(t151, 0.0_f64, 2.0_f64 / 3.0_f64 * t741 * t1289);
    let t1362 = piecewise3(t155, 0.0_f64, -2.0_f64 / 3.0_f64 * t745 * t1289);
    let t1364 = t1359 / 2.0_f64 + t1362 / 2.0_f64;
    (t1342, t1344, t1351, t1352, t1353, t1354, t1356, t1364)
}
