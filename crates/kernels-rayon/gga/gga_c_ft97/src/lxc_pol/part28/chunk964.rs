//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 964/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk964(t32594: f64, t8392: f64, t1851: f64, t7281: f64, t1882: f64, t32475: f64, t487: f64, t7211: f64, t7276: f64, t8232: f64, t32607: f64, t32610: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t137768 = t8392 * t32594;
    let t137797 = t1851 * t7281;
    let t137802 = t1882 * t32475;
    let t137804 = t487 * t7211;
    let t137810 = 8.0_f64 / 27.0_f64 * t8232 * t7276;
    let t137812 = t8392 * t32607;
    let t137814 = t8392 * t32610;
    (t137768, t137797, t137802, t137804, t137810, t137812, t137814)
}
