//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 382/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk382(t1180: f64, t127: f64, t1441: f64, t1446: f64, t1451: f64, t1456: f64, t1462: f64, t1466: f64, t1470: f64, t1490: f64, t1494: f64, t1498: f64, t1503: f64, t1507: f64, t1511: f64, t1514: f64, t1516: f64, t1526: f64, t1531: f64, t1535: f64, t335: f64, t367: f64, t418: f64) -> f64 {
    let t1538 = 0.17149607247227894789e-2_f64 * t418 * t1441 - 0.17149607247227894789e-2_f64 * t418 * t1446 - 0.85748036236139473944e-3_f64 * t418 * t1451 - 0.85748036236139473944e-3_f64 * t418 * t1456 + 0.12862205435420921092e-2_f64 * t418 * t1462 - 0.42874018118069736972e-3_f64 * t1180 * t1466 + 0.42874018118069736972e-3_f64 * t1180 * t1470 + t127 * t1490 / 96.0_f64 - t335 * t1494 / 48.0_f64 - t335 * t1498 / 48.0_f64 - t335 * t1503 / 48.0_f64 - t367 * t1507 / 96.0_f64 - t367 * t1511 / 96.0_f64 + 7.0_f64 / 288.0_f64 * t1514 + 7.0_f64 / 144.0_f64 * t1516 - t367 * t1526 / 96.0_f64 + 0.42874018118069736972e-3_f64 * t1531 * t1535;
    t1538
}
