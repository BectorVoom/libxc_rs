//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1240/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1240(t12234: f64, t16836: f64, t1943: f64, t531: f64, t27357: f64, t16823: f64, t27370: f64, t94229: f64, t1394: f64, t16810: f64, t7923: f64, t11814: f64, t28516: f64) -> (f64, f64, f64, f64) {
    let t98239 = t16836 * t12234;
    let t98240 = t1943 * t531;
    let t98242 = t98239 * t98240 * t27357;
    let t98246 = t27370 * t16823 * t94229;
    let t98252 = t1394 * t7923 * t16810;
    let t98254 = t11814 * t28516;
    (t98242, t98246, t98252, t98254)
}
