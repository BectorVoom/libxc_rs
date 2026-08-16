//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 915/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk915(t2347: f64, t2567: f64, t5070: f64, t8232: f64, t5087: f64, t5066: f64, t2492: f64, t5132: f64, t5153: f64, t222: f64, t2382: f64, t226: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t65313 = t2567 * t2347;
    let t65327 = t8232 * t5070;
    let t65437 = t8232 * t5087;
    let t65508 = t8232 * t5066;
    let t65592 = t2492 * t5132;
    let t65644 = t8232 * t5153;
    let t65692 = t2382 * t222;
    let t65693 = t65692 * t226;
    (t65313, t65327, t65437, t65508, t65592, t65644, t65693)
}
