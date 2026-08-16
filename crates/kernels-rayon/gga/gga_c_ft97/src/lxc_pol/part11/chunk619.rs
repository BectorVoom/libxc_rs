//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 619/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk619(t7954: f64, t82: f64, t110: f64, t7955: f64, t1651: f64, t447: f64, t499: f64, t1901: f64, t446: f64, t8526: f64, t8529: f64, t8534: f64, t8536: f64, t8541: f64, t8546: f64, t8551: f64, t8555: f64, t8559: f64, t8564: f64, t8567: f64, t8570: f64, t8574: f64) -> (f64, f64, f64, f64) {
    let t8577 = t7954 * t82;
    let t8579 = t8577 * t110 * t7955;
    let t8583 = t447 * t499 * t1651;
    let t8586 = t8526 / 9.0_f64 + 2.0_f64 * t446 * t8529 - t8534 + 2.0_f64 / 3.0_f64 * t1901 * t8536 - 2.0_f64 * t446 * t8541 + 2.0_f64 * t446 * t8546 + t446 * t8551 + t446 * t8555 - 2.0_f64 / 3.0_f64 * t1901 * t8559 - 2.0_f64 * t446 * t8564 + 2.0_f64 / 9.0_f64 * t8567 - 2.0_f64 / 9.0_f64 * t446 * t8570 - t446 * t8574 / 9.0_f64 - 10.0_f64 / 81.0_f64 * t446 * t8579 - t446 * t8583 / 3.0_f64;
    (t8577, t8579, t8583, t8586)
}
