//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1049/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1049<F: Float>(t1077: F, t5967: F, t1081: F, t6055: F, t1: F, t397: F, t6011: F, t339: F, t6069: F, t2357: F, t39: F, t1217: F, t2455: F) -> (F, F, F, F, F, F) {
    let t18973 = t5967 * t1077;
    let t18976 = t6055 * t1081;
    let t18981 = t6011 * t1 * t397;
    let t18998 = t339 * t6069;
    let t19008 = t39 * t2357;
    let t19123 = t2455 * t1217;
    (t18973, t18976, t18981, t18998, t19008, t19123)
}
