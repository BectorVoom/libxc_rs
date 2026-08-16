//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 398/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk398(t143: f64, t160: f64, t2133: f64, t376: f64, t599: f64, t89: f64, t597: f64, t604: f64) -> (f64, f64, f64) {
    let t2135 = t143 * t2133 * t160;
    let t2140 = t89 * t376 * t599;
    let t2142 = t597 * t604;
    (t2135, t2140, t2142)
}
