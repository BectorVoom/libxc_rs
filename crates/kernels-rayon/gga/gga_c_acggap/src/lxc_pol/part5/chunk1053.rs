//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1053/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1053(t1165: f64, t3346: f64, t3361: f64, t540: f64, t14047: f64, t4908: f64, t4680: f64, t4907: f64, t1140: f64, t4773: f64, t4430: f64, t3375: f64, t4959: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18480 = t3361 * t1165 * t540 * t3346;
    let t18482 = t14047 * t4908;
    let t18485 = t3361 * t4680 * t4907;
    let t18487 = t1140 * t4773;
    let t18489 = t1140 * t4430;
    let t18499 = t3375 * t4959;
    (t18480, t18482, t18485, t18487, t18489, t18499)
}
