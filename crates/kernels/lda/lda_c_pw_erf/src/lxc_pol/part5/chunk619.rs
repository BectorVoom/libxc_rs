//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 619/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk619<F: Float>(t164: F, t5446: F, t1901: F, t479: F, t1905: F, t163: F, t169: F, t2198: F, t299: F, t717: F, t780: F, t1138: F, t1597: F, t1124: F, t483: F, t485: F) -> (F, F, F, F, F, F, F, F) {
    let t5448 = 0.06301081444628223 * t5446 * t164;
    let t5449 = t1901 * t479;
    let t5455 = 0.06301081444628223 * t1905 * t479;
    let t5459 = 0.017961351015381915 * t169 * t299 * t2198 * t163;
    let t5466 = t717 * t780;
    let t5468 = t5466 * t1138 * t1597;
    let t5470 = t1124 * t780;
    let t5472 = t5470 * t483 * t485;
    (t5448, t5449, t5455, t5459, t5466, t5468, t5470, t5472)
}
