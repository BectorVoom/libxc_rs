//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1294/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1294<F: Float>(t1969: F, t5220: F, t1981: F, t1982: F, t5312: F, t4602: F, t6536: F, t1444: F, t6282: F, t10216: F, t2469: F, t493: F) -> (F, F, F, F, F) {
    let t16992 = t5220 * t1969;
    let t16993 = F::new(8.0) / F::new(45.0) * t16992;
    let t16996 = F::new(8.0) / F::new(45.0) * t1981 * t5312 * t1982;
    let t16998 = F::new(8.0) / F::new(45.0) * t4602 * t6536;
    let t17000 = F::new(2.0) / F::new(27.0) * t1444 * t6282;
    let t17003 = t493 * t10216 * t2469 / F::new(27.0);
    (t16993, t16996, t16998, t17000, t17003)
}
