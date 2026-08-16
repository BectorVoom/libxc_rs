//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 894/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk894(t342: f64, t630: f64, t7729: f64, t344: f64, t8639: f64, t7800: f64, t81: f64, t1526: f64, t7705: f64, t7721: f64, t1533: f64, t2252: f64) -> (f64, f64, f64, f64, f64) {
    let t38341 = t342 * t630 * t7729;
    let t38355 = 5.0_f64 / 54.0_f64 * t342 * t8639 * t344;
    let t38357 = t81 * t7800;
    let t38366 = t1526 * t7705 * t7721;
    let t38369 = t342 * t2252 * t1533;
    (t38341, t38355, t38357, t38366, t38369)
}
