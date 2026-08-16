//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 251/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk251(t265: f64, t713: f64, t729: f64, t251: f64, t249: f64, t458: f64, t241: f64, t665: f64) -> (f64, f64, f64, f64) {
    let t731 = t729 * t265 * t713;
    let t734 = 1.0_f64 / t251;
    let t736 = t458 * t249 / 3.0_f64;
    let t737 = t665 * t241;
    (t731, t734, t736, t737)
}
