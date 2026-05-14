//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 928/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk928<F: Float>(t1125: F, t153: F, t2357: F, t632: F, t7045: F, t15483: F, t242: F, t7032: F, t1143: F, t2379: F, t6138: F, t2594: F, t2765: F, t440: F, t7199: F, t7191: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19361 = t153 * t1125 * t2357;
    let t19363 = t7045 * t632;
    let t19365 = t15483 * t242;
    let t19385 = t7032 * t632;
    let t19388 = t2379 * t1143;
    let t19397 = t6138 * t242;
    let t19421 = t2765 * t2594 * t440;
    let t19425 = t2765 * t7199;
    let t19449 = t2765 * t7191;
    (t19361, t19363, t19365, t19385, t19388, t19397, t19421, t19425, t19449)
}
