//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2793/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2793(t16558: f64, t707: f64, t751: f64, t16586: f64, t9929: f64, t185: f64, t55677: f64, t16579: f64, t172: f64, t763: f64, t67: f64, t758: f64) -> (f64, f64, f64, f64, f64) {
    let t59037 = t707 * t751 * t16558;
    let t59038 = 8.0_f64 * t59037;
    let t59039 = t9929 * t16586;
    let t59040 = 24.0_f64 * t59039;
    let t59043 = 4.0_f64 * t707 * t185 * t55677;
    let t59045 = t16579 * t172 * t763;
    let t59046 = 0.11696447245269292414e1_f64 * t59045;
    let t59048 = t16579 * t67 * t758;
    (t59038, t59040, t59043, t59046, t59048)
}
