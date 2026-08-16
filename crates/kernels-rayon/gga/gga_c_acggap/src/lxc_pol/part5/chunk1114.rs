//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1114/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1114(t19942: f64, t19966: f64, t59: f64, t40: f64, t87: f64, t85: f64, t14930: f64, t14935: f64, t11721: f64, t11708: f64, t19451: f64, t19452: f64, t19453: f64, t19454: f64, t19455: f64, t19456: f64, t19914: f64, t19915: f64, t19916: f64, t19917: f64, t19918: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19968 = (t19942 + t19966) * t59;
    let t19970 = t40 * t19968 * t87;
    let t19972 = 0.19751673498613801407e-1_f64 * t19968 * t85;
    let t19973 = 4.0_f64 * t14930;
    let t19974 = 2.0_f64 * t14935;
    let t19975 = 0.20779030926817756511e3_f64 * t11721;
    let t19976 = -t19451 + t19452 + t19453 + t19454 - t19455 + t19456 + t19914 - t19915 + t19916 + t19917 + t11708 - t19918 + t19970 + t19972 + t19973 + t19974 + t19975;
    (t19970, t19972, t19973, t19974, t19975, t19976)
}
