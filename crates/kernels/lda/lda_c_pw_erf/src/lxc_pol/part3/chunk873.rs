//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 873/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk873<F: Float>(t11322: F, t8189: F, t1769: F, t4295: F, t2851: F, t749: F, t1: F, t397: F, t4383: F, t11305: F, t11308: F, t11310: F, t11311: F, t11312: F, t11314: F, t11316: F, t11318: F, t11320: F, t8168: F, t8177: F, t8184: F, t8188: F) -> (F, F, F, F, F) {
    let t11323 = 0.032530742648344574 * t11322;
    let t11324 = 1.7544670192365612 * t8189;
    let t11325 = t1769 * t4295;
    let t11327 = t2851 * t749;
    let t11328 = 144.0 * t11327;
    let t11330 = t4383 * t1 * t397;
    let t11331 = 0.0005493466511025948 * t11330;
    let t11332 = -0.4740006021527056 * t11305 + t11308 - t11310 - t8168 - t8177 - t11311 - t11312 - t11314 - t11316 + t11318 + t11320 + t11323 + t8184 - t8188 - t11324 - 1.825614615114074 * t11325 - t11328 - t11331;
    (t11323, t11324, t11328, t11331, t11332)
}
