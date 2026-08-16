//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 936/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk936(t8914: f64, t8915: f64, t935: f64, t450: f64, t3101: f64, t8912: f64, t3107: f64, t1128: f64, t3128: f64, t1121: f64, t3245: f64, t8493: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8916 = t8914 * t8915;
    let t8917 = t8916 * t935;
    let t8918 = t450 * t8917;
    let t8921 = t3101 * t8912;
    let t8922 = t8914 * t3107;
    let t8923 = t8922 * t935;
    let t8924 = t450 * t8923;
    let t8927 = t1128 * t3128;
    let t8928 = t1121 * t8927;
    let t8930 = t3245 * t8493;
    (t8917, t8918, t8921, t8923, t8924, t8928, t8930)
}
