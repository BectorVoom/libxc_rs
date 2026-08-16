//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 999/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk999(t40196: f64, t12054: f64, t9333: f64, t12065: f64, t2437: f64, t2441: f64, t38759: f64, t895: f64, t10348: f64, t13779: f64, t1407: f64, t9285: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t47926 = 0.85206502119823888171e-1_f64 * t40196;
    let t47927 = t12054 * t9333;
    let t47934 = t2437 * t12065;
    let t47937 = t2441 * t12065;
    let t47939 = t895 * t38759;
    let t47941 = t12054 * t10348;
    let t47949 = t1407 * t13779;
    let t47951 = t9285 * t12065;
    (t47926, t47927, t47934, t47937, t47939, t47941, t47949, t47951)
}
