//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1052/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1052<F: Float>(t168: F, t635: F, t7025: F, t1125: F, t153: F, t2357: F, t632: F, t7045: F, t15483: F, t242: F, t7032: F, t1143: F, t2379: F) -> (F, F, F, F, F, F) {
    let t19358 = t168 * t635 * t7025;
    let t19361 = t153 * t1125 * t2357;
    let t19363 = t7045 * t632;
    let t19365 = t15483 * t242;
    let t19385 = t7032 * t632;
    let t19388 = t2379 * t1143;
    (t19358, t19361, t19363, t19365, t19385, t19388)
}
