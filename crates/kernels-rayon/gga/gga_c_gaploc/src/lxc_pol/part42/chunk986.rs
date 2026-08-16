//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 986/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk986(t1029: f64, t12255: f64, t14373: f64, t14415: f64, t14428: f64, t14431: f64, t1897: f64, t1901: f64, t2508: f64, t2580: f64, t2958: f64, t3009: f64, t38907: f64, t39403: f64, t43042: f64, t44828: f64, t44829: f64, t44837: f64, t44883: f64, t44887: f64, t44895: f64, t47652: f64, t50051: f64, t50150: f64, t702: f64, t7129: f64, t7137: f64, t7226: f64, t8670: f64) -> f64 {
    let t50407 = t44828 - 0.85450291446024714264e-3_f64 * t44829 + 0.64087718584518535698e-3_f64 * t44837 + t44883 - t44887 + 0.41016139894091862845e-1_f64 * t7137 * t14415 - 0.23071578690426672851e-1_f64 * t2508 * t1901 * t50150 + 0.15381052460284448567e-1_f64 * t2508 * t2580 * t50051 + 0.38452631150711121419e-2_f64 * t43042 + 0.46143157380853345702e-1_f64 * t1897 * t12255 * t8670 + 0.18457262952341338281e0_f64 * t2508 * t2580 * t2958 * t38907 - 0.92286314761706691402e-1_f64 * t2508 * t7226 * t3009 * t38907 - 0.76905262301422242837e-2_f64 * t1897 * t14373 * t702 + 0.20508069947045931423e-1_f64 * t7137 * t14431 + 0.17090058289204942853e-2_f64 * t47652 - 0.46143157380853345702e-1_f64 * t7129 * t14428 - 0.46143157380853345702e-1_f64 * t2508 * t39403 * t1029 + t44895;
    t50407
}
