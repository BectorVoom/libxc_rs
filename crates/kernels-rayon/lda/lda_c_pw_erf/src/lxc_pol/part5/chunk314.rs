//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 314/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk314(t1098: f64, t169: f64, t242: f64, t299: f64, t465: f64, t632: f64, t699: f64, t145: f64, t943: f64, t1067: f64, t703: f64, t461: f64, t8: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1101 = 0.14149184788746388_f64 * t169 * t1098 * t242;
    let t1102 = t299 * t465;
    let t1104 = t169 * t1102 * t242;
    let t1108 = 0.10611888591559791_f64 * t169 * t699 * t632;
    let t1109 = 2.0_f64 * t145;
    let t1110 = 8.0_f64 * t943;
    let t1111 = 6.0_f64 * t1067;
    let t1118 = t169 * t703 * t632;
    let t1124 = 1.0_f64 / t8 / t461;
    (t1101, t1102, t1104, t1108, t1109, t1110, t1111, t1118, t1124)
}
