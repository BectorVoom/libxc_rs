//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1267/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1267<F: Float>(t1243: F, t1953: F, t21246: F, t1251: F, t348: F, t7360: F, t1245: F, t325: F, t7652: F, t7644: F, t331: F, t7767: F) -> (F, F, F, F, F, F) {
    let t22747 = t1953 * t1243 * t21246;
    let t22759 = t1251 * t7360 * t348;
    let t22764 = t1245 * t7360 * t348;
    let t22786 = t325 * t7652;
    let t22788 = t325 * t7644;
    let t22790 = t331 * t7767;
    (t22747, t22759, t22764, t22786, t22788, t22790)
}
