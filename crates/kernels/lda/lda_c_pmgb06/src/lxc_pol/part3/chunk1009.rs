//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1009/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1009<F: Float>(t1080: F, t4865: F, t1915: F, t1981: F, t2924: F, t493: F, t6751: F, t1444: F, t5487: F, t1992: F, t3457: F, t1586: F, t529: F, t851: F) -> (F, F, F, F, F) {
    let t11997 = t4865 * t1080;
    let t12000 = F::new(4.0) / F::new(5.0) * t1981 * t1915 * t11997;
    let t12003 = t493 * t6751 * t2924 / F::new(9.0);
    let t12005 = F::new(2.0) / F::new(15.0) * t1444 * t5487;
    let t12006 = t1992 * t3457;
    let t12011 = F::new(3.0) / F::new(5.0) * t493 * t12006 * t851 * t1586 * t529;
    (t11997, t12000, t12003, t12005, t12011)
}
