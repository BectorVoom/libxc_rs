//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 351/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk351<F: Float>(t1251: F, t940: F, t503: F, t11: F, t504: F, t945: F, t191: F, t299: F, t187: F, t190: F, t331: F, t539: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1252 = t1251 * t940;
    let t1253 = t503 * t1252;
    let t1254 = t11 * t1253;
    let t1256 = t504 * t945;
    let t1257 = t503 * t1256;
    let t1258 = t11 * t1257;
    let t1260 = t299 * t191;
    let t1263 = F::new(0.011111111111111112) * t190 * t1260 * t187;
    let t1264 = t331 * t539;
    (t1252, t1253, t1254, t1256, t1257, t1258, t1260, t1263, t1264)
}
