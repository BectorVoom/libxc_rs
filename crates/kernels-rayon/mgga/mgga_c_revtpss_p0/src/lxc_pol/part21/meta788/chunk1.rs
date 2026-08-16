//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2837/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2837(t15162: f64, t698: f64, t141: f64, t2908: f64, t51873: f64, t15165: f64, t51847: f64, t930: f64, t41246: f64, t41267: f64, t41275: f64, t51921: f64, t51923: f64, t51927: f64, t51932: f64, t51935: f64) -> (f64, f64, f64, f64, f64) {
    let t51937 = t698 * t15162;
    let t51940 = t141 * t2908 * t51873;
    let t51942 = t698 * t15165;
    let t51945 = t141 * t930 * t51847;
    let t51949 = 0.55190000000000000001e-1_f64 * t51921 + 0.73586666666666666668e-1_f64 * t51923 - 0.82785e-1_f64 * t51927 - 0.11038e0_f64 * t51932 - 0.27595e-1_f64 * t51935 - 0.33114000000000000001e0_f64 * t51937 - 0.99342e0_f64 * t51940 + 0.99342e0_f64 * t51942 + 0.198684e1_f64 * t51945 + t41246 - 0.33114e0_f64 * t41267 + 0.33114e0_f64 * t41275;
    (t51937, t51940, t51942, t51945, t51949)
}
