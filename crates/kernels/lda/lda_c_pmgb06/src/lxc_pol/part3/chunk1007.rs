//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1007/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1007<F: Float>(t1602: F, t1831: F, t1981: F, t2871: F, t153: F, t1864: F, t3216: F, t439: F, t1444: F, t5333: F, t4861: F, t493: F, t5447: F) -> (F, F, F, F) {
    let t11981 = F::new(4.0) / F::new(15.0) * t1981 * t2871 * t1831 * t1602;
    let t11985 = F::new(2.0) / F::new(15.0) * t439 * t3216 * t153 * t1864;
    let t11987 = F::new(2.0) / F::new(5.0) * t1444 * t5333;
    let t11990 = F::new(2.0) / F::new(5.0) * t493 * t5447 * t4861;
    (t11981, t11985, t11987, t11990)
}
