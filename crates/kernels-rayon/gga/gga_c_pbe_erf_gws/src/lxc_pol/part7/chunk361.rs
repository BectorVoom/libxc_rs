//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 361/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk361(t1425: f64, t40: f64, t414: f64, t428: f64, t461: f64, t409: f64, t1: f64, t427: f64, t467: f64, t1328: f64, t1330: f64, t1394: f64, t1398: f64, t1401: f64, t1424: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1426 = t40 * t1425;
    let t1427 = 2.0_f64 * t1426;
    let t1428 = t414 * t428;
    let t1429 = 8.0_f64 * t1428;
    let t1430 = t414 * t461;
    let t1431 = 8.0_f64 * t1430;
    let t1432 = t409 * t428;
    let t1433 = 8.0_f64 * t1432;
    let t1434 = t427 * t1;
    let t1435 = t1434 * t467;
    let t1436 = 0.36623110073506319882e-3_f64 * t1435;
    let t1437 = -t1394 - t1398 - t1401 + t1328 + t1424 + t1427 - t1429 - t1431 + t1433 + t1330 - t1436;
    (t1427, t1429, t1431, t1433, t1434, t1436, t1437)
}
