//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1138/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1138<F: Float>(t1472: F, t6414: F, t3416: F, t6405: F, t1287: F, t1318: F, t1319: F, t2415: F, t1954: F, t2065: F, t4841: F, t571: F, t1446: F, t6419: F, t15811: F, t519: F, t5250: F) -> (F, F, F, F, F, F) {
    let t16740 = 16.0 / 15.0 * t1472 * t6414;
    let t16742 = 32.0 / 45.0 * t3416 * t6405;
    let t16746 = 16.0 / 45.0 * t1318 * t1319 * t2415 * t1287;
    let t16750 = 32.0 / 45.0 * t571 * t4841 * t1954 * t2065;
    let t16752 = 64.0 / 81.0 * t1446 * t6419;
    let t16755 = 32.0 / 81.0 * t519 * t5250 * t15811;
    (t16740, t16742, t16746, t16750, t16752, t16755)
}
