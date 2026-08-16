//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2623/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2623(t47060: f64, t13581: f64, t72: f64, t757: f64, t47073: f64, t5635: f64, t9586: f64, t5571: f64, t9425: f64, t47078: f64, t9318: f64, t1857: f64, t9342: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t48275 = 0.35089341735807877242e1_f64 * t47060;
    let t48277 = t13581 * t72 * t757;
    let t48278 = 0.54934341918019635162e-3_f64 * t48277;
    let t48279 = 8.0_f64 * t47073;
    let t48280 = t5635 * t9586;
    let t48281 = 0.56968947174242584612e-3_f64 * t48280;
    let t48282 = t5571 * t9425;
    let t48283 = 0.35089341735807877242e1_f64 * t48282;
    let t48284 = 0.18311447306006545054e-3_f64 * t47078;
    let t48285 = t5571 * t9318;
    let t48286 = 0.35089341735807877242e1_f64 * t48285;
    let t48287 = t9342 * t1857;
    (t48275, t48278, t48279, t48281, t48283, t48284, t48286, t48287)
}
