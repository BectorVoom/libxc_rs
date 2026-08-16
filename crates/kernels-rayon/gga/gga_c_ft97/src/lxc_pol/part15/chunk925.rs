//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 925/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk925(t52916: f64, t2252: f64, t342: f64, t4910: f64, t1526: f64, t42262: f64, t4906: f64, t5454: f64, t8640: f64, t5450: f64, t5459: f64, t5470: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t68774 = 56.0_f64 / 81.0_f64 * t52916;
    let t69073 = t342 * t2252 * t4910;
    let t69137 = t1526 * t42262 * t4906;
    let t69265 = t8640 * t5454;
    let t69289 = t8640 * t5450;
    let t69291 = t8640 * t5459;
    let t69374 = t8640 * t5470;
    (t68774, t69073, t69137, t69265, t69289, t69291, t69374)
}
