//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 906/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk906(t1775: f64, t8322: f64, t8311: f64, t8319: f64, t8328: f64, t8295: f64, t8292: f64, t11761: f64, t11762: f64, t1787: f64, t3127: f64, t3134: f64, t37259: f64, t37264: f64, t37269: f64, t37279: f64, t37287: f64, t37324: f64, t38277: f64, t38526: f64, t462: f64, t8291: f64, t8327: f64) -> f64 {
    let t38598 = t1775 * t8322;
    let t38600 = t1775 * t8311;
    let t38602 = t1775 * t8319;
    let t38604 = t1775 * t8328;
    let t38606 = t1775 * t8295;
    let t38614 = t1775 * t8292;
    let t38631 = 4.0_f64 / 3.0_f64 * t462 * t1787 * t37279 - 4.0_f64 / 3.0_f64 * t38598 - 8.0_f64 / 3.0_f64 * t38600 + 8.0_f64 / 9.0_f64 * t38602 - 8.0_f64 / 9.0_f64 * t38604 + 8.0_f64 / 3.0_f64 * t38606 + 8.0_f64 * t462 * t1787 * t37287 + 4.0_f64 / 3.0_f64 * t462 * t8327 * t37324 + 8.0_f64 / 3.0_f64 * t38614 - 4.0_f64 * t462 * t8291 * t38277 + 4.0_f64 / 3.0_f64 * t462 * t1787 * t37259 + 8.0_f64 / 3.0_f64 * t462 * t3134 * t37264 - 8.0_f64 / 9.0_f64 * t462 * t3127 * t37269 - 8.0_f64 * t11761 * t11762 * t38526;
    t38631
}
