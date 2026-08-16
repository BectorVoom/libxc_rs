//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 646/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk646(t8189: f64, t1636: f64, t433: f64, t89: f64, t1557: f64, t487: f64, t1586: f64, t355: f64, t100: f64, t1541: f64, t443: f64, t444: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8190 = 14.0_f64 / 81.0_f64 * t8189;
    let t8192 = t89 * t1636 * t433;
    let t8210 = t487 * t1557;
    let t8216 = t355 * t1586;
    let t8217 = t8216 * t100;
    let t8232 = t443 * t444 * t1541;
    (t8190, t8192, t8210, t8216, t8217, t8232)
}
