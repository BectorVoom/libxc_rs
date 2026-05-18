//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1149/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1149<F: Float>(t11966: F, t28346: F, t189: F, t1899: F, t15508: F, t90: F, t18680: F, t277: F, t327: F, t34169: F, t34171: F, t34174: F, t34176: F, t34178: F, t34181: F, t34184: F, t34188: F, t34191: F) -> (F, F, F) {
    let t34193 = t11966 * t28346;
    let t34195 = t189 * t1899;
    let t34197 = t15508 * t90;
    let t34200 = t277 * t34195 * t34197 * t327 * t18680;
    let t34202 = F::new(0.10567613244746075633e-6) * t34169 + F::new(0.33764099580923002116e-6) * t34171 + F::new(0.67528199161846004232e-6) * t34174 + F::new(0.18115908419564701086e-6) * t34176 + F::new(0.36231816839129402172e-6) * t34178 - F::new(0.2318836277704281739e-4) * t34181 + F::new(0.21720231316129303386e-4) * t34184 + F::new(0.1718991074781972522e-8) * t34188 - F::new(0.1422820120100248667e-7) * t34191 + F::new(0.36897447374131944446e-6) * t34193 - F::new(0.19263878310735033706e-7) * t34200;
    (t34195, t34197, t34202)
}
