//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 928/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk928(t8826: f64, t8841: f64, t1075: f64, t1065: f64, t2972: f64, t393: f64, t2975: f64, t401: f64, t8787: f64, t1085: f64, t3029: f64, t8639: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8842 = t8826 + t8841;
    let t8843 = t8842 * t1075;
    let t8847 = 1.0_f64 / t2972 / t1065;
    let t8848 = t393 * t8847;
    let t8850 = 1.0_f64 / t2975 / t401;
    let t8851 = t8787 * t8850;
    let t8854 = t3029 * t1085;
    let t8857 = 0.28842592592592592592e-1_f64 * t8639;
    (t8842, t8843, t8847, t8848, t8850, t8851, t8854, t8857)
}
