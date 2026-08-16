//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 872/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk872(t2101: f64, t2179: f64, t157: f64, t40436: f64, t604: f64, t7763: f64, t143: f64, t38052: f64, t161: f64, t38061: f64, t89: f64, t40424: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40911 = t2101 * t2179;
    let t40926 = t40436 * t157;
    let t40931 = t604 * t7763;
    let t41002 = t38052 * t143;
    let t41093 = 280.0_f64 / 243.0_f64 * t89 * t38061 * t161;
    let t41251 = t40424 * t157;
    (t40911, t40926, t40931, t41002, t41093, t41251)
}
