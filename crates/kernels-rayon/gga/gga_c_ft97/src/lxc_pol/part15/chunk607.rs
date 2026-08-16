//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 607/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk607(t8232: f64, t981: f64, t110: f64, t8326: f64, t1780: f64, t488: f64, t1637: f64, t89: f64, t973: f64, t1771: f64, t963: f64, t2: f64, t8275: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11550 = t8232 * t981;
    let t11552 = t8326 * t110;
    let t11556 = t1780 * t488;
    let t11578 = t89 * t1637 * t973;
    let t11669 = t1771 * t963;
    let t11690 = t8275 * t2;
    (t11550, t11552, t11556, t11578, t11669, t11690)
}
