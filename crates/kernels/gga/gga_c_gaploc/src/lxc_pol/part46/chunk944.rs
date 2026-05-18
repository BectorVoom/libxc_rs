//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 944/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk944<F: Float>(t13191: F, t7137: F, t2508: F, t33285: F, t7659: F, t3276: F, t8682: F, t8503: F, t9739: F, t28953: F, t9014: F, t1897: F, t2580: F, t28236: F, t2958: F) -> (F, F, F, F, F, F) {
    let t43173 = F::new(0.12304841968227558854e0) * t7137 * t13191;
    let t43175 = t2508 * t33285 * t7659;
    let t43179 = F::new(0.11535789345213336425e0) * t2508 * t3276 * t8682;
    let t43182 = F::new(0.38452631150711121418e0) * t2508 * t9739 * t8503;
    let t43185 = F::new(0.18457262952341338281e0) * t2508 * t9014 * t28953;
    let t43189 = F::new(0.15381052460284448567e-1) * t1897 * t2580 * t2958 * t28236;
    (t43173, t43175, t43179, t43182, t43185, t43189)
}
