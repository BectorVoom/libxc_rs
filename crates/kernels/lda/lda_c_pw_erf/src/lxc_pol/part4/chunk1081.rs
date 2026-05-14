//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1081/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1081<F: Float>(t9251: F, t5129: F, t795: F, t12874: F, t2002: F, t4760: F, t4763: F, t4738: F, t5310: F, t10397: F, t1403: F, t2411: F, t571: F, t1318: F, t4794: F, t6370: F) -> (F, F, F, F, F, F, F) {
    let t15735 = 4.0 / 135.0 * t9251;
    let t15737 = 4.0 / 15.0 * t795 * t5129;
    let t15740 = 32.0 / 45.0 * t12874 * t2002;
    let t15742 = 32.0 / 45.0 * t4763 * t4760;
    let t15743 = t4738 * t5310;
    let t15744 = 64.0 / 135.0 * t15743;
    let t15748 = 8.0 / 27.0 * t571 * t10397 * t2411 * t1403;
    let t15750 = t1318 * t4794 * t6370;
    (t15735, t15737, t15740, t15742, t15744, t15748, t15750)
}
