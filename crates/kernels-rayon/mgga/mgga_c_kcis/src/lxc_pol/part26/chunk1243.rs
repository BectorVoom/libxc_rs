//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1243/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1243(t28383: f64, t3728: f64, t28357: f64, t4142: f64, t28510: f64, t3717: f64, t52460: f64, t12234: f64, t16836: f64, t1943: f64, t531: f64, t11814: f64, t28516: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t98162 = t3728 * t28383;
    let t98193 = t4142 * t28357;
    let t98225 = t4142 * t28510;
    let t98226 = 0.14739506172839506172e-2_f64 * t98225;
    let t98233 = t52460 * t3717;
    let t98239 = t16836 * t12234;
    let t98240 = t1943 * t531;
    let t98254 = t11814 * t28516;
    (t98162, t98193, t98225, t98226, t98233, t98239, t98240, t98254)
}
