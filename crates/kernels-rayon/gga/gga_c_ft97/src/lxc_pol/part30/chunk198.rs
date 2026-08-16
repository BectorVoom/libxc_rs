//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 198/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk198(t1477: f64, t317: f64, t193: f64, t1476: f64, t319: f64, t840: f64, t845: f64, t91: f64, t26: f64) -> (f64, f64, f64, f64, f64) {
    let t1478 = t1477 * t317;
    let t1479 = t193 * t1478;
    let t1483 = t840 * t319 * t1476;
    let t1485 = t91 * t845;
    let t1486 = t1485 * t26;
    (t1478, t1479, t1483, t1485, t1486)
}
