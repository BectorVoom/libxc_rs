//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1086/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1086(t14892: f64, t192: f64, t5506: f64, t14898: f64, t14900: f64, t14902: f64, t14904: f64, t11683: f64, t11696: f64, t234: f64, t34: f64, t821: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19441 = 0.65061487801810439052e-1_f64 * t14892;
    let t19444 = t192 * t5506;
    let t19451 = 0.36622894612013090108e-3_f64 * t14898;
    let t19452 = 0.97661052298701573622e-3_f64 * t14900;
    let t19453 = 0.2077903092681775651e3_f64 * t14902;
    let t19454 = 0.46785788981077169656e1_f64 * t14904;
    let t19455 = 0.70178683471615754484e1_f64 * t11683;
    let t19456 = 12.0_f64 * t11696;
    let t19461 = t234 * t34 * t821;
    (t19441, t19444, t19451, t19452, t19453, t19454, t19455, t19456, t19461)
}
