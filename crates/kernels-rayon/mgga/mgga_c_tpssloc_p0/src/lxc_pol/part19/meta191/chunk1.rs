//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 851/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk851(t10121: f64, t193: f64, t202: f64, t2379: f64, t2522: f64, t2523: f64, t2553: f64, t262: f64, t4314: f64, t766: f64, t776: f64, t870: f64, t9450: f64, t9457: f64, t9458: f64, t9463: f64, t9469: f64, t9470: f64, t9476: f64, t9484: f64, t9496: f64, t9516: f64) -> f64 {
    let t10125 = t10121 * t193 * t202 * t870 + 6.0_f64 * t193 * t262 * t9458 + 3.0_f64 * t193 * t766 * t9516 + 18.0_f64 * t2379 * t2523 * t4314 + 9.0_f64 * t2522 * t2523 * t2553 - 9.0_f64 * t2522 * t776 * t9470 + t9450 - t9457 + t9463 - t9469 + t9476 + t9484 - t9496;
    t10125
}
