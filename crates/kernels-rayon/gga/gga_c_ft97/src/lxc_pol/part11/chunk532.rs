//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 532/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk532(t327: f64, t703: f64, t230: f64, t113: f64, t332: f64, t38: f64, t401: f64, t6: f64, t77: f64, t51: f64, t78: f64, t388: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4334 = t703 * t327;
    let t4342 = t230 * t327;
    let t4381 = t332 * t113;
    let t5517 = t38 * t401;
    let t5536 = t77 * t6;
    let t5537 = t5536 * t51;
    let t5544 = t78 * t6;
    let t5545 = t388 * t5544;
    (t4334, t4342, t4381, t5517, t5537, t5544, t5545)
}
