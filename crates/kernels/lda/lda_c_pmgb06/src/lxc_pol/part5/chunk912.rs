//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 912/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk912<F: Float>(t19251: F, t1423: F, t7577: F, t7581: F, t16343: F, t806: F, t2007: F, t6127: F, t1980: F, t2012: F, t2591: F, t1444: F, t7674: F, t16794: F, t493: F, t834: F) -> (F, F, F, F, F, F, F, F) {
    let t19252 = 2.0 / 27.0 * t19251;
    let t19253 = t1423 * t7577;
    let t19254 = 2.0 / 27.0 * t19253;
    let t19255 = t1423 * t7581;
    let t19256 = 2.0 / 135.0 * t19255;
    let t19258 = t16343 * t806 / 15.0;
    let t19260 = t6127 * t2007 / 15.0;
    let t19263 = 2.0 / 15.0 * t2591 * t1980 * t2012;
    let t19265 = t1444 * t7674 / 15.0;
    let t19268 = t493 * t16794 * t834 / 15.0;
    (t19252, t19254, t19256, t19258, t19260, t19263, t19265, t19268)
}
