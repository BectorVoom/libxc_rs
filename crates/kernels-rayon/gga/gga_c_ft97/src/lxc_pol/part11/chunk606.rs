//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 606/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk606(t1647: f64, t447: f64, t499: f64, t103: f64, t1755: f64, t379: f64, t1902: f64, t375: f64, t443: f64, t444: f64) -> (f64, f64, f64, f64, f64) {
    let t8383 = t447 * t499 * t1647;
    let t8386 = t103 * t1755;
    let t8387 = t8386 * t379;
    let t8388 = t1902 * t8387;
    let t8392 = t443 * t444 * t375;
    (t8383, t8386, t8387, t8388, t8392)
}
