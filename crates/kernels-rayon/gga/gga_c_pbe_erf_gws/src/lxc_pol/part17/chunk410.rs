//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 410/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk410(t1435: f64, t1328: f64, t1330: f64, t1394: f64, t1398: f64, t1401: f64, t1424: f64, t1427: f64, t1429: f64, t1431: f64, t1433: f64, t408: f64, t413: f64) -> (f64, f64, f64) {
    let t1436 = 0.36623110073506319882e-3_f64 * t1435;
    let t1437 = -t1394 - t1398 - t1401 + t1328 + t1424 + t1427 - t1429 - t1431 + t1433 + t1330 - t1436;
    let t1438 = t408 * t413;
    (t1436, t1437, t1438)
}
