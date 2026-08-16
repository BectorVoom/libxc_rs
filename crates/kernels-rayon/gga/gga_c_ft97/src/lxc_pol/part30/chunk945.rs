//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 945/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk945(t33282: f64, t33284: f64, t33288: f64, t33312: f64, t6109: f64, t681: f64, t1434: f64, t33347: f64, t2360: f64, t7440: f64, t7484: f64, t33481: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t141282 = t33282 * t33288 * t33284;
    let t141295 = t6109 * t681 * t33312;
    let t141304 = t1434 * t681 * t33347;
    let t141314 = t7440 * t2360;
    let t141319 = t7484 * t2360;
    let t141340 = t89 * t681 * t33481;
    (t141282, t141295, t141304, t141314, t141319, t141340)
}
