//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 646/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk646<F: Float>(t440: F, t770: F, t2765: F, t4465: F, t2455: F, t668: F, t2325: F, t406: F, t2329: F, t92: F, t2334: F, t408: F, t2337: F, t93: F, t108: F, t2268: F, t2274: F, t348: F, t352: F, t5992: F, t6005: F, t659: F, t661: F, t943: F) -> (F, F, F, F, F, F, F) {
    let t6155 = t770 * t440;
    let t6156 = t2765 * t6155;
    let t6161 = 8.0 / 135.0 * t4465;
    let t6162 = t2455 * t668;
    let t6164 = t406 * t2325;
    let t6169 = t92 * t2329;
    let t6174 = t408 * t2334;
    let t6179 = t93 * t2337;
    let t6185 = (40.0 / 27.0 * t6164 * t348 + 80.0 / 9.0 * t2268 * t943 + 20.0 / 9.0 * t6169 * t348 + 4.0 / 3.0 * t659 * t5992 + 40.0 / 27.0 * t6174 * t352 - 80.0 / 9.0 * t2274 * t943 + 20.0 / 9.0 * t6179 * t352 + 4.0 / 3.0 * t661 * t6005) * t108;
    (t6155, t6156, t6161, t6162, t6164, t6174, t6185)
}
