//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3290/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3290(t14141: f64, t23037: f64, t686: f64, t72: f64, t10049: f64, t22863: f64, t22954: f64, t4118: f64, t47967: f64, t47971: f64, t47979: f64, t47981: f64, t47985: f64, t74935: f64, t74943: f64, t74945: f64, t820: f64) -> f64 {
    let t86401 = t14141 * t23037 * t72 * t686;
    let t86405 = -0.58911598146606471821e-3_f64 * t47967 - 0.65854491829355115987e0_f64 * t820 * t4118 * t22954 + 0.91069445034239308177e-1_f64 * t47971 - t47979 - t47981 + 0.39512695097613069591e1_f64 * t820 * t10049 * t22863 - 0.65854491829355115984e-1_f64 * t74935 + 0.43902994552903410658e-1_f64 * t47985 + 0.58544643236296698112e-1_f64 * t86401 + 0.32927245914677557992e-1_f64 * t74943 + 0.39029762157531132074e-2_f64 * t74945;
    t86405
}
