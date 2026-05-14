//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1072/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1072<F: Float>(t108: F, t267: F, t821: F, t4518: F, t4523: F, t12314: F, t5157: F, t518: F, t6850: F, t2193: F, t3416: F, t6190: F, t1401: F, t6843: F, t1466: F, t571: F, t593: F) -> (F, F, F, F, F, F, F, F) {
    let t15607 = t821 * t108 * t267;
    let t15609 = 32.0 / 45.0 * t15607 * t4518;
    let t15611 = 16.0 / 27.0 * t15607 * t4523;
    let t15613 = 32.0 / 45.0 * t12314 * t5157;
    let t15614 = t6850 * t518;
    let t15616 = 16.0 / 15.0 * t15614 * t2193;
    let t15618 = 8.0 / 15.0 * t3416 * t6190;
    let t15619 = t1401 * t6843;
    let t15623 = 8.0 / 15.0 * t571 * t1466 * t15619 * t593;
    (t15607, t15609, t15611, t15613, t15614, t15616, t15618, t15623)
}
