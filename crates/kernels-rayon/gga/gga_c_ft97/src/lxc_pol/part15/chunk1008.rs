//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1008/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1008(t378: f64, t85474: f64, t92: f64, t1570: f64, t85451: f64, t85456: f64, t37355: f64, t85469: f64, t38052: f64, t358: f64, t85501: f64, t38063: f64, t45304: f64, t59002: f64, t59007: f64, t73975: f64, t73977: f64, t73985: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t85544 = t92 * t378 * t85474;
    let t85546 = t1570 * t85451;
    let t85548 = t92 * t378 * t85546;
    let t85551 = t92 * t378 * t85456;
    let t85554 = t37355 * t85469;
    let t85556 = t92 * t38052 * t85554;
    let t85558 = t358 * t85501;
    let t85560 = t92 * t378 * t85558;
    let t85567 = -12.0_f64 * t85544 + 2.0_f64 * t85548 + 8.0_f64 / 3.0_f64 * t85551 + 4.0_f64 / 9.0_f64 * t73985 - 80.0_f64 / 81.0_f64 * t85556 - t85560 / 3.0_f64 + t38063 + 112.0_f64 / 81.0_f64 * t45304 - 16.0_f64 / 27.0_f64 * t59002 + 16.0_f64 / 9.0_f64 * t59007 - 16.0_f64 / 9.0_f64 * t73975 + 8.0_f64 / 3.0_f64 * t73977;
    (t85544, t85546, t85548, t85551, t85554, t85556, t85558, t85560, t85567)
}
