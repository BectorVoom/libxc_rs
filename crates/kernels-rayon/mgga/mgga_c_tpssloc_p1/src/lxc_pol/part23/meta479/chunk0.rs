//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1434/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1434(t11285: f64, t1164: f64, t44154: f64, t78287: f64, t22233: f64, t4869: f64, t21830: f64, t11282: f64, t3403: f64, t18915: f64, t6106: f64, t6270: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t78310 = 0.12304822629859687989e5_f64 * t1164 * t44154 * t78287 * t11285;
    let t78312 = 0.23392894490538584828e1_f64 * t4869 * t22233;
    let t78314 = 0.20779030926817756511e3_f64 * t4869 * t21830;
    let t78318 = 0.6233709278045326953e3_f64 * t1164 * t11282 * t78287 * t3403;
    let t78320 = 0.10389515463408878255e3_f64 * t18915 * t6106;
    let t78321 = t6270 * t6270;
    (t78310, t78312, t78314, t78318, t78320, t78321)
}
