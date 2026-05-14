//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 776/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk776<F: Float>(t2508: F, t32658: F, t954: F, t40746: F, t40750: F, t40752: F, t40758: F, t13185: F, t7129: F, t13217: F, t13191: F, t7137: F, t3276: F, t8682: F, t8503: F, t9739: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t43146 = 0.15381052460284448567e-1 * t2508 * t954 * t32658;
    let t43148 = 0.64087718584518535698e-3 * t40746;
    let t43152 = 0.64087718584518535698e-3 * t40750;
    let t43156 = 0.64087718584518535698e-3 * t40752;
    let t43157 = 0.64087718584518535698e-3 * t40758;
    let t43166 = 0.53833683610995569986e-1 * t7129 * t13185;
    let t43168 = 0.46143157380853345701e-1 * t7129 * t13217;
    let t43173 = 0.12304841968227558854e0 * t7137 * t13191;
    let t43179 = 0.11535789345213336425e0 * t2508 * t3276 * t8682;
    let t43182 = 0.38452631150711121418e0 * t2508 * t9739 * t8503;
    (t43146, t43148, t43152, t43156, t43157, t43166, t43168, t43173, t43179, t43182)
}
