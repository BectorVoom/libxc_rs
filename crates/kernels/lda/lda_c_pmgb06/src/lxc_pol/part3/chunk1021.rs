//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1021/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1021<F: Float>(t1972: F, t2988: F, t1420: F, t5242: F, t439: F, t4672: F, t5225: F, t12382: F, t1897: F, t5233: F, t2956: F, t5482: F, t3453: F, t831: F, t1499: F, t2095: F) -> (F, F, F, F, F, F, F, F) {
    let t13997 = t1972 * t2988 / 15.0;
    let t13999 = 2.0 / 15.0 * t1420 * t5242;
    let t14002 = 2.0 / 15.0 * t439 * t5225 * t4672;
    let t14005 = 2.0 / 45.0 * t439 * t1897 * t12382;
    let t14007 = 2.0 / 15.0 * t1420 * t5233;
    let t14010 = t439 * t5482 * t2956 / 15.0;
    let t14011 = t831 * t3453;
    let t14012 = t14011 / 15.0;
    let t14014 = t1499 * t2095 / 10.0;
    (t13997, t13999, t14002, t14005, t14007, t14010, t14012, t14014)
}
