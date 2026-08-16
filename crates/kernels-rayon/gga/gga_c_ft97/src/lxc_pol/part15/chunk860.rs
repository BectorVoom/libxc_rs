//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 860/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk860(t7763: f64, t81: f64, t342: f64, t344: f64, t8639: f64, t7800: f64, t37292: f64, t1586: f64, t22: f64, t36452: f64, t37991: f64, t96: f64) -> (f64, f64, f64, f64, f64) {
    let t38327 = t81 * t7763;
    let t38355 = 5.0_f64 / 54.0_f64 * t342 * t8639 * t344;
    let t38357 = t81 * t7800;
    let t38392 = 280.0_f64 / 81.0_f64 * t37292;
    let t38456 = 1.0_f64 / t96 / t37991 / t22 / t1586 / t36452 / 96.0_f64;
    (t38327, t38355, t38357, t38392, t38456)
}
