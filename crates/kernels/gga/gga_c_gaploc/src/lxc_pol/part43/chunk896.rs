//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 896/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk896<F: Float>(t13185: F, t7129: F, t13217: F, t13191: F, t7137: F, t2508: F, t3276: F, t8682: F, t8503: F, t9739: F, t28953: F, t9014: F) -> (F, F, F, F, F, F) {
    let t43166 = F::new(0.53833683610995569986e-1) * t7129 * t13185;
    let t43168 = F::new(0.46143157380853345701e-1) * t7129 * t13217;
    let t43173 = F::new(0.12304841968227558854e0) * t7137 * t13191;
    let t43179 = F::new(0.11535789345213336425e0) * t2508 * t3276 * t8682;
    let t43182 = F::new(0.38452631150711121418e0) * t2508 * t9739 * t8503;
    let t43185 = F::new(0.18457262952341338281e0) * t2508 * t9014 * t28953;
    (t43166, t43168, t43173, t43179, t43182, t43185)
}
