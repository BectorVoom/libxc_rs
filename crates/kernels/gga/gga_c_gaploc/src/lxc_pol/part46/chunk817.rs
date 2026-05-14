//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 817/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk817<F: Float>(t2508: F, t3276: F, t8682: F, t8503: F, t9739: F, t28953: F, t9014: F, t1897: F, t2580: F, t28236: F, t2958: F, t40775: F, t1022: F, t6058: F, t28668: F, t5241: F) -> (F, F, F, F, F, F) {
    let t43179 = 0.11535789345213336425e0 * t2508 * t3276 * t8682;
    let t43182 = 0.38452631150711121418e0 * t2508 * t9739 * t8503;
    let t43185 = 0.18457262952341338281e0 * t2508 * t9014 * t28953;
    let t43189 = 0.15381052460284448567e-1 * t1897 * t2580 * t2958 * t28236;
    let t43190 = 0.1922631557535556071e-2 * t40775;
    let t43191 = t6058 * t1022;
    let t43195 = 0.46143157380853345701e0 * t2508 * t43191 * t5241 * t28668;
    (t43179, t43182, t43185, t43189, t43190, t43195)
}
