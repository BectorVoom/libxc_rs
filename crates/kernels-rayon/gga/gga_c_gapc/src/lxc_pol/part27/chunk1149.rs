//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1149/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1149(t11966: f64, t28346: f64, t189: f64, t1899: f64, t15508: f64, t90: f64, t18680: f64, t277: f64, t327: f64, t34169: f64, t34171: f64, t34174: f64, t34176: f64, t34178: f64, t34181: f64, t34184: f64, t34188: f64, t34191: f64) -> (f64, f64, f64) {
    let t34193 = t11966 * t28346;
    let t34195 = t189 * t1899;
    let t34197 = t15508 * t90;
    let t34200 = t277 * t34195 * t34197 * t327 * t18680;
    let t34202 = 0.10567613244746075633e-6_f64 * t34169 + 0.33764099580923002116e-6_f64 * t34171 + 0.67528199161846004232e-6_f64 * t34174 + 0.18115908419564701086e-6_f64 * t34176 + 0.36231816839129402172e-6_f64 * t34178 - 0.2318836277704281739e-4_f64 * t34181 + 0.21720231316129303386e-4_f64 * t34184 + 0.1718991074781972522e-8_f64 * t34188 - 0.1422820120100248667e-7_f64 * t34191 + 0.36897447374131944446e-6_f64 * t34193 - 0.19263878310735033706e-7_f64 * t34200;
    (t34195, t34197, t34202)
}
