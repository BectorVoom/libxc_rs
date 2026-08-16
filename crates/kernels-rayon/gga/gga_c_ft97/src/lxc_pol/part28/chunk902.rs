//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 902/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk902(t76: f64, t8050: f64, t378: f64, t7241: f64, t1586: f64, t1642: f64, t22: f64, t36452: f64, t37991: f64, t96: f64, t1554: f64, t355: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t38241 = 1.0_f64 / t8050 / t76;
    let t38262 = t378 * t7241;
    let t38268 = t1642 * t1586;
    let t38456 = 1.0_f64 / t96 / t37991 / t22 / t1586 / t36452 / 96.0_f64;
    let t38463 = t1554 * t1586;
    let t38477 = t355 * t7241;
    (t38241, t38262, t38268, t38456, t38463, t38477)
}
