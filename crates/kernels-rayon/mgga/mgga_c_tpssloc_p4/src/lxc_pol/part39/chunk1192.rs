//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1192/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1192(t3375: f64, t4857: f64, t1157: f64, t1164: f64, t3400: f64, t4883: f64, t3411: f64, t4884: f64, t225: f64, t4947: f64, t4943: f64, t1734: f64, t3590: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14960 = t3375 * t4857;
    let t14961 = t14960 * t1157;
    let t14963 = 0.23392894490538584828e1_f64 * t1164 * t14961;
    let t14966 = t3400 * t4857;
    let t14967 = t14966 * t4883;
    let t14969 = 0.34631718211362927518e2_f64 * t1164 * t14967;
    let t14971 = 0.34631718211362927518e2_f64 * t3411 * t4884;
    let t14972 = t4947 * t225;
    let t14980 = t4943 * t225;
    let t14985 = t3590 * t1734;
    (t14963, t14969, t14971, t14972, t14980, t14985)
}
