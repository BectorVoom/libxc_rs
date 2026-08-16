//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1014/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1014(t1526: f64, t1944: f64, t38308: f64, t1970: f64, t7705: f64, t8779: f64, t11262: f64, t8767: f64, t342: f64, t630: f64, t8783: f64, t142: f64, t7800: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41332 = t1526 * t38308 * t1944;
    let t41335 = t1526 * t7705 * t1970;
    let t41338 = t1526 * t7705 * t8779;
    let t41341 = t1526 * t11262 * t8767;
    let t41344 = t342 * t630 * t8783;
    let t41349 = t142 * t7800;
    (t41332, t41335, t41338, t41341, t41344, t41349)
}
