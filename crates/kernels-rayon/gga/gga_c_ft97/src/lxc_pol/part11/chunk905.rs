//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 905/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk905(t2: f64, t37355: f64, t37357: f64, t1797: f64, t8282: f64, t1783: f64, t1793: f64, t1775: f64, t8316: f64, t8276: f64, t8267: f64, t1587: f64, t1780: f64, t38550: f64, t38554: f64, t38556: f64, t38560: f64, t38562: f64, t38566: f64, t38570: f64, t462: f64, t463: f64, t8183: f64, t8261: f64, t8275: f64) -> (f64, f64, f64) {
    let t38571 = t2 * t37355;
    let t38572 = t38571 * t37357;
    let t38576 = t8282 * t1797;
    let t38578 = t8282 * t1783;
    let t38584 = t8282 * t1793;
    let t38586 = t1775 * t8316;
    let t38588 = t8276 * t37357;
    let t38592 = t1775 * t8267;
    let t38594 = 40.0_f64 / 9.0_f64 * t462 * t8275 * t38550 + 112.0_f64 / 81.0_f64 * t38554 - 2.0_f64 / 3.0_f64 * t462 * t1780 * t38556 + 8.0_f64 / 3.0_f64 * t38560 + 8.0_f64 * t462 * t463 * t38562 + 2.0_f64 * t462 * t463 * t38566 - 80.0_f64 / 81.0_f64 * t462 * t38570 * t38572 - 8.0_f64 / 9.0_f64 * t38576 - 16.0_f64 / 27.0_f64 * t38578 + 8.0_f64 * t462 * t1587 * t8261 * t8183 + 16.0_f64 / 9.0_f64 * t38584 - 16.0_f64 / 9.0_f64 * t38586 - 8.0_f64 * t462 * t1780 * t38588 + 4.0_f64 / 9.0_f64 * t38592;
    (t38572, t38588, t38594)
}
