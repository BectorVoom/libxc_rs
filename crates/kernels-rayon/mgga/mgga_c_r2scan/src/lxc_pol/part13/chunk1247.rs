//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1247/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1247(t322: f64, t40892: f64, t40923: f64, t40954: f64, t40986: f64, t41019: f64, t41054: f64, t41086: f64, t2449: f64, t3461: f64, t374: f64, t40505: f64, t40509: f64, t40526: f64, t40528: f64, t40532: f64, t40536: f64, t40539: f64, t40541: f64, t40544: f64, t40547: f64, t40551: f64, t40554: f64, t40569: f64, t40571: f64, t40578: f64) -> f64 {
    let t323 = t322 <= 0.0_f64;
    let t331 = t322 <= 0.25e1_f64;
    let t41088 = piecewise5(t323, t40892, t331, t40923 + t40954 + t40986 + t41019, t41054 + t41086);
    let t41090 = 2.0_f64 * t2449 * t3461 + t374 * t41088 + t40505 - t40509 - t40526 - t40528 - t40532 - t40536 - t40539 + t40541 + t40544 + t40547 + t40551 - t40554 + t40569 + t40571 - t40578;
    t41090
}
