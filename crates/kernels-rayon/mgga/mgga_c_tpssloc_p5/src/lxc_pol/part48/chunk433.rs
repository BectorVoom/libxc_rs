//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 433/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk433(t3046: f64, t364: f64, t354: f64, t1043: f64, t121: f64, t248: f64, t884: f64, t1041: f64, t1044: f64, t2780: f64, t283: f64, t883: f64) -> (f64, f64, f64, f64) {
    let t3047 = t364 * t3046;
    let t3048 = t354 * t3047;
    let t3051 = t121 * t1043;
    let t3053 = t248 * t3051 * t884;
    let t3054 = t1041 * t3053;
    let t3057 = t248 * t1044 * t2780;
    let t3061 = 1.0_f64 / t283 / t883;
    (t3048, t3054, t3057, t3061)
}
