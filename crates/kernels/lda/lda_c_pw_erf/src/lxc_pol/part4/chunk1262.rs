//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1262/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1262<F: Float>(t163: F, t169: F, t299: F, t7287: F, t2668: F, t717: F, t473: F, t483: F, t485: F, t6039: F, t1131: F, t7220: F, t2363: F, t1138: F, t1597: F, t164: F, t6138: F) -> (F, F, F, F, F, F, F) {
    let t18761 = t169 * t299 * t7287 * t163;
    let t18765 = t169 * t717 * t2668 * t163;
    let t18779 = t473 * t6039 * t483 * t485;
    let t18782 = t7220 * t1131 * t485;
    let t18784 = t717 * t2363;
    let t18786 = t18784 * t1138 * t1597;
    let t18788 = t6138 * t164;
    (t18761, t18765, t18779, t18782, t18784, t18786, t18788)
}
