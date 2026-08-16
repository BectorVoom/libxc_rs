//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 448/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk448(t3040: f64, t360: f64, t1021: f64, t248: f64, t1030: f64, t372: f64, t364: f64, t354: f64, t1043: f64, t121: f64, t884: f64, t1041: f64) -> (f64, f64, f64, f64, f64) {
    let t3041 = t3040 * t360;
    let t3043 = t248 * t1021 * t3041;
    let t3046 = t1030 * t372;
    let t3047 = t364 * t3046;
    let t3048 = t354 * t3047;
    let t3051 = t121 * t1043;
    let t3053 = t248 * t3051 * t884;
    let t3054 = t1041 * t3053;
    (t3043, t3046, t3048, t3053, t3054)
}
