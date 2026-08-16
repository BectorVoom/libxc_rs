//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1060/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1060(t21239: f64, t4271: f64, t7706: f64, t14496: f64, t14497: f64, t30153: f64, t30875: f64, t416: f64, t30273: f64, t6287: f64, t30294: f64, t6279: f64) -> (f64, f64, f64, f64, f64) {
    let t31352 = t4271 * t21239 * t7706;
    let t31356 = t14496 * t14497 * t30153;
    let t31379 = t416 * t30875;
    let t31385 = t6287 * t30273;
    let t31388 = t6279 * t30294;
    (t31352, t31356, t31379, t31385, t31388)
}
