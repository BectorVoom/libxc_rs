//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 510/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk510(t548: f64, t135: f64, t23: f64, t7368: f64, t1642: f64, t525: f64, t1984: f64, t378: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8906 = t548 * t548;
    let t8907 = 1.0_f64 / t8906;
    let t8908 = t135 * t8907;
    let t9016 = t23 * t7368;
    let t9049 = t1642 * t525;
    let t9073 = t378 * t1984;
    (t8906, t8907, t8908, t9016, t9049, t9073)
}
