//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 909/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk909(t100: f64, t1586: f64, t10: f64, t16: f64, t1642: f64, t369: f64, t2035: f64, t39: f64, t538: f64, t355: f64, t929: f64, t526: f64, t597: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47660 = t1586 * t100;
    let t47666 = t10 * t16 * t1642;
    let t47667 = t369 * t100;
    let t48841 = t538 * t39 * t2035;
    let t48917 = t355 * t929;
    let t49414 = t526 * t597;
    (t47660, t47666, t47667, t48841, t48917, t49414)
}
