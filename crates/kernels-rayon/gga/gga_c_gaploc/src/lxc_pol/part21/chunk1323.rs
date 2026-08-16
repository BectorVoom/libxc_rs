//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1323/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1323(t34582: f64, t34363: f64, t587: f64, t912: f64, t10615: f64, t31158: f64, t32033: f64, t6963: f64, t6964: f64, t10526: f64, t20471: f64, t6540: f64, t986: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34583 = 0.38342925953920749676e0_f64 * t34582;
    let t34585 = t587 * t912 * t34363;
    let t34586 = 0.19171462976960374838e0_f64 * t34585;
    let t34587 = t10615 * t31158;
    let t34588 = 0.17875244975925213335e0_f64 * t34587;
    let t34592 = 0.85801175884441024006e1_f64 * t6963 * t6964 * t32033;
    let t34595 = 0.42900587942220512002e1_f64 * t20471 * t10526 * t32033;
    let t34600 = t6540 * t986;
    (t34583, t34586, t34588, t34592, t34595, t34600)
}
