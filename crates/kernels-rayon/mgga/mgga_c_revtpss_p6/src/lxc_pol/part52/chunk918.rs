//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 918/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk918(t1967: f64, t816: f64, t1014: f64, t65: f64, t4579: f64, t3252: f64, t4574: f64, t3204: f64, t7131: f64, t4817: f64, t7132: f64, t25517: f64, t25543: f64, t25551: f64, t25557: f64, t25560: f64, t25564: f64, t4783: f64, t4788: f64, t4839: f64) -> f64 {
    let t27526 = t1967 * t816;
    let t27527 = t65 * t1014;
    let t27528 = t27527 * t4579;
    let t27531 = t65 * t3252;
    let t27532 = t27531 * t4574;
    let t27536 = t3204 * t7131;
    let t27539 = t7132 * t4817;
    let t27541 = 0.28582678745379824648e-3_f64 * t25517 * t4783 + 0.28582678745379824648e-3_f64 * t25517 * t4788 + t25543 / 864.0_f64 + 0.19055119163586549765e-3_f64 * t25551 - 0.15244095330869239812e-2_f64 * t25557 - t27526 * t27528 / 144.0_f64 + t27526 * t27532 / 216.0_f64 - t25560 + 0.28582678745379824648e-3_f64 * t25564 + 0.85748036236139473944e-3_f64 * t27536 * t4839 + 0.19055119163586549765e-3_f64 * t27539;
    t27541
}
