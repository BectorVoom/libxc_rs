//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 925/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk925<F: Float>(t169: F, t18784: F, t242: F, t6035: F, t632: F, t6040: F, t1143: F, t2364: F, t1085: F, t2343: F, t4: F, t1077: F, t5967: F, t1081: F, t6055: F, t1: F, t397: F, t6011: F) -> (F, F, F, F, F, F, F, F) {
    let t18923 = t169 * t18784 * t242;
    let t18934 = t169 * t6035 * t632;
    let t18942 = t169 * t6040 * t632;
    let t18945 = t169 * t2364 * t1143;
    let t18965 = t2343 * t4 * t1085;
    let t18973 = t5967 * t1077;
    let t18976 = t6055 * t1081;
    let t18981 = t6011 * t1 * t397;
    (t18923, t18934, t18942, t18945, t18965, t18973, t18976, t18981)
}
