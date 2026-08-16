//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 426/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk426(t259: f64, t379: f64, t1474: f64, t1464: f64, t366: f64, t220: f64, t368: f64, t983: f64, t985: f64, t981: f64, t373: f64, t978: f64, t1402: f64, t1413: f64, t1427: f64, t1453: f64, t1455: f64, t1459: f64, t198: f64, t330: f64, t995: f64, param_beta: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t380 = t259 < t379;
    let t1475 = param_beta * t1474;
    let t1477 = t366 * t1464;
    let t1482 = t1474 * t220 * t368 + t1477 * t983 * t985;
    let t1483 = t981 * t1482;
    let t1485 = t1475 * t373 - t1483 * t978;
    let t1490 = piecewise3(t380, t1485 * t198 * t330 * t995 - t1413 + t1427 + t1453 + t1455 - t1459, t1402);
    (t1475, t1477, t1482, t1483, t1485, t1490)
}
