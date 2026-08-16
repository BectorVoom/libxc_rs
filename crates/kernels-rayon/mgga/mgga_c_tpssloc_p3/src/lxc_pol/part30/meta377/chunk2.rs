//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1436/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1436(t5660: f64, t870: f64, t12850: f64, t12860: f64, t16577: f64, t16578: f64, t16581: f64, t16582: f64, t16583: f64, t16588: f64, t16589: f64, t16592: f64, t16596: f64, t2522: f64, t2523: f64, t4119: f64, t4307: f64, t4310: f64, t4314: f64, t5544: f64, t776: f64, t9457: f64, t9469: f64, t9476: f64, t9484: f64, t9496: f64) -> f64 {
    let t16606 = t5660 * t870;
    let t16610 = 6.0_f64 * t16592 * t4314 * t776 - 6.0_f64 * t16596 * t2522 * t4307 + 3.0_f64 * t16606 * t2522 * t776 + 3.0_f64 * t2522 * t2523 * t5544 + 6.0_f64 * t2522 * t4119 * t4310 + 12.0_f64 * t16583 * t4314 + 6.0_f64 * t16589 * t4314 + t12850 - t12860 + t16577 + t16578 + t16581 + t16582 + t16588 - t9457 - t9469 + t9476 + t9484 - t9496;
    t16610
}
