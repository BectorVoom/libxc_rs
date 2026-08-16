//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 554/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk554(t7800: f64, t82: f64, t1586: f64, t378: f64, t12: f64, t52: f64, t25: f64, t409: f64, t29: f64, t31: f64, t122: f64, t170: f64, t7239: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7801 = t82 * t7800;
    let t7824 = t378 * t1586;
    let t7853 = t52 * t12;
    let t7876 = t409 * t25;
    let t7905 = 1.0_f64 / t31 / t29;
    let t7906 = t122 * t7905;
    let t7911 = 4.0_f64 * t170 * t7239;
    (t7801, t7824, t7853, t7876, t7906, t7911)
}
