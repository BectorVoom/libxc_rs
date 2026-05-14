//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1116/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1116<F: Float>(t100: F, t2594: F, t163: F, t169: F, t299: F, t7851: F, t11621: F, t11626: F, t11627: F, t15481: F, t15484: F, t15486: F, t9178: F, t9180: F, t9181: F, t9192: F, t9195: F, t9203: F, t9206: F, t9211: F) -> (F, F) {
    let t23124 = t2594 * t100;
    let t23150 = t169 * t299 * t7851 * t163;
    let t23152 = t9178 - t9180 - 0.00011865309871651405 * t9181 - 0.09451622166942335 * t15481 + 0.09451622166942335 * t15484 + 0.1890324433388467 * t15486 - 0.1890324433388467 * t9192 - t9195 + 0.09451622166942335 * t9203 + t9206 + 0.0878110494085338 * t9211 - t11621 + t11626 + 0.2835486650082701 * t11627 + 0.008980675507690957 * t23150;
    (t23124, t23152)
}
