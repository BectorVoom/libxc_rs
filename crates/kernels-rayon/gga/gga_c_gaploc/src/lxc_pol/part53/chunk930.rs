//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 930/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk930(t41337: f64, t13077: f64, t28439: f64, t32744: f64, t9824: f64, t10924: f64, t1980: f64, t41342: f64, t13072: f64, t32969: f64, t10867: f64, t41511: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43910 = 0.11916829983950142223e0_f64 * t41337;
    let t43912 = t13077 * t28439;
    let t43913 = 0.59584149919750711116e-1_f64 * t43912;
    let t43914 = t32744 * t9824;
    let t43915 = 0.29792074959875355558e-1_f64 * t43914;
    let t43917 = t1980 * t10924 * t9824;
    let t43918 = 0.29792074959875355558e-1_f64 * t43917;
    let t43924 = 0.29792074959875355558e-1_f64 * t41342;
    let t43925 = t32969 * t13072;
    let t43927 = t10867 * t41511;
    (t43910, t43913, t43915, t43918, t43924, t43925, t43927)
}
