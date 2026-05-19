//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1187/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1187<F: Float>(t10548: F, t789: F, t421: F, t5900: F, t1147: F, t794: F, t1193: F, t1354: F, t1798: F, t740: F, t409: F, t419: F, t4463: F) -> (F, F, F, F, F, F, F) {
    let t14270 = t789 * t10548;
    let t14275 = t5900 * t421;
    let t14277 = t1147 * t794;
    let t14279 = t14277 * t1193 * t1354;
    let t14281 = t740 * t1798;
    let t14283 = t14281 * t1193 * t1354;
    let t14284 = F::cast_from(0.0014862827083471494_f64) * t14283;
    let t14287 = t409 * t4463 * t419 * t421;
    (t14270, t14275, t14277, t14279, t14281, t14284, t14287)
}
