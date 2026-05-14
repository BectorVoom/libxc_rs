//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 989/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk989<F: Float>(t479: F, t5451: F, t1590: F, t1905: F, t164: F, t4437: F, t1191: F, t163: F, t169: F, t841: F, t2198: F, t717: F, t299: F, t5433: F, t1318: F, t2192: F, t9432: F) -> (F, F, F, F, F, F, F) {
    let t11640 = t5451 * t479;
    let t11642 = t1905 * t1590;
    let t11644 = t4437 * t164;
    let t11652 = t169 * t1191 * t841 * t163;
    let t11666 = t169 * t717 * t2198 * t163;
    let t11670 = t169 * t299 * t5433 * t163;
    let t11677 = t1318 * t9432 * t2192;
    (t11640, t11642, t11644, t11652, t11666, t11670, t11677)
}
