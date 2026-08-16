//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 610/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk610(t11043: f64, t11076: f64, t100: f64, t8275: f64, t103: f64, t7763: f64, t7800: f64, t1851: f64, t358: f64, t1073: f64, t8680: f64, t1570: f64, t2266: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11939 = 4.0_f64 / 27.0_f64 * t11043;
    let t11949 = 4.0_f64 / 9.0_f64 * t11076;
    let t11987 = t8275 * t100;
    let t11988 = t103 * t7763;
    let t12020 = t103 * t7800;
    let t12045 = t1851 * t358;
    let t12112 = t8680 * t1073;
    let t12116 = t2266 * t1570;
    (t11939, t11949, t11987, t11988, t12020, t12045, t12112, t12116)
}
