//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1173/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1173<F: Float>(t1444: F, t5477: F, t1972: F, t2988: F, t1420: F, t5242: F, t439: F, t4672: F, t5225: F, t12382: F, t1897: F, t5233: F) -> (F, F, F, F, F, F) {
    let t13995 = F::new(2.0) / F::new(15.0) * t1444 * t5477;
    let t13997 = t1972 * t2988 / F::new(15.0);
    let t13999 = F::new(2.0) / F::new(15.0) * t1420 * t5242;
    let t14002 = F::new(2.0) / F::new(15.0) * t439 * t5225 * t4672;
    let t14005 = F::new(2.0) / F::new(45.0) * t439 * t1897 * t12382;
    let t14007 = F::new(2.0) / F::new(15.0) * t1420 * t5233;
    (t13995, t13997, t13999, t14002, t14005, t14007)
}
