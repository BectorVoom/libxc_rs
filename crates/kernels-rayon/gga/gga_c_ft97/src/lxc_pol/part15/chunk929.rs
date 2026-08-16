//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 929/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk929(t19233: f64, t287: f64, t19106: f64, t4092: f64, t1771: f64, t5360: f64, t5356: f64, t5352: f64, t8282: f64, t5346: f64, t5349: f64, t1636: f64, t5226: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t70671 = t19233 * t287;
    let t70779 = t4092 * t19106;
    let t70799 = t1771 * t5360;
    let t70801 = t1771 * t5356;
    let t70826 = t8282 * t5352;
    let t70935 = t8282 * t5346;
    let t70999 = t8282 * t5349;
    let t71238 = t89 * t1636 * t5226;
    (t70671, t70779, t70799, t70801, t70826, t70935, t70999, t71238)
}
