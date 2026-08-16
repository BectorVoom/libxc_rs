//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 760/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk760(t32338: f64, t432: f64, t28: f64, t89: f64, t5507: f64, t5617: f64, t375: f64, t7260: f64, t358: f64, t7211: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32339 = t32338 * t432;
    let t32340 = t28 * t32339;
    let t32341 = t89 * t32340;
    let t32343 = t5507 * t5617;
    let t32344 = t28 * t32343;
    let t32345 = t89 * t32344;
    let t32348 = t89 * t375 * t7260;
    let t32349 = t32348 / 9.0_f64;
    let t32350 = t7211 * t358;
    (t32339, t32341, t32343, t32345, t32348, t32349, t32350)
}
