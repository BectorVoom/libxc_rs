//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 450/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk450(t1353: f64, t1414: f64, t828: f64, t1368: f64, t1370: f64, t1372: f64, t1378: f64, t1383: f64, t1388: f64, t1401: f64, t1407: f64, t1410: f64) -> (f64, f64) {
    let t1416 = t1414 * t828 * t1353;
    let t1419 = -t1368 - t1370 * t1372 / 48.0_f64 - t1378 + t1383 - 0.21437009059034868486e-3_f64 * t1388 * t1401 - t1407 - 0.85748036236139473944e-3_f64 * t1410 * t1416;
    (t1416, t1419)
}
