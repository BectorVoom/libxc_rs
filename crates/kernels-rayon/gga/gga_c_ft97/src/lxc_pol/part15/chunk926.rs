//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 926/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk926(t39448: f64, t5446: f64, t5442: f64, t8640: f64, t5315: f64, t8232: f64, t1636: f64, t5300: f64, t89: f64, t5343: f64, t8282: f64, t4939: f64, t801: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t69468 = t39448 * t5446;
    let t69510 = t8640 * t5442;
    let t70000 = t8232 * t5315;
    let t70141 = t89 * t1636 * t5300;
    let t70142 = 4.0_f64 / 9.0_f64 * t70141;
    let t70231 = t8282 * t5343;
    let t70278 = t4939 * t801;
    (t69468, t69510, t70000, t70141, t70142, t70231, t70278)
}
