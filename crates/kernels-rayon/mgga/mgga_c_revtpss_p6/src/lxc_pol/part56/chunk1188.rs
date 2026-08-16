//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1188/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1188(t29127: f64, t33468: f64, t105270: f64, t124604: f64, t124605: f64, t124626: f64, t124665: f64, t124671: f64, t124819: f64, t124915: f64, t124927: f64, t1294: f64, t131631: f64, t131657: f64, t26948: f64, t29179: f64, t31993: f64, t33471: f64, t33477: f64, t33478: f64, t34908: f64, t34940: f64, t34960: f64, t3719: f64, t5230: f64, t5422: f64, t7637: f64, t7652: f64, t8945: f64, t8948: f64) -> f64 {
    let t131826 = t33468 * t29127;
    let t131849 = 0.6854368519812282314e1_f64 * t33477 * t124915 * t34960 * t1294 - 0.1859366460452550541e-3_f64 * t131657 * t8945 * t8948 - 0.17135921299530705785e1_f64 * t124671 * t34940 - 0.17135921299530705785e1_f64 * t131826 * t33471 + 0.11156198762715303246e-2_f64 * t124819 * t31993 * t3719 * t131631 - 0.34694512752820797848e1_f64 * t124626 * t7652 * t5422 - t124927 + 0.34694512752820797848e1_f64 * t124605 * t7637 * t105270 - 0.52041769129231196772e1_f64 * t26948 * t124604 * t7637 * t5230 - 0.34271842599061411569e1_f64 * t33477 * t33478 * t34908 * t1294 + 0.34694512752820797848e1_f64 * t124665 * t29179;
    t131849
}
