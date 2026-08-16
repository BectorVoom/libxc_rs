//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1021/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1021<F: Float>(t1858: F, t3115: F, t1901: F, t439: F, t1074: F, t4654: F, t2010: F, t10203: F, t153: F, t3010: F, t760: F, t9190: F) -> (F, F, F, F, F, F) {
    let t12146 = t1858 * t3115;
    let t12149 = t439 * t1901 * t12146 / F::cast_from(27.0_f64);
    let t12150 = t4654 * t1074;
    let t12153 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2010 * t1901 * t12150;
    let t12154 = t10203 * t153;
    let t12156 = t9190 * t760 * t3010;
    (t12146, t12149, t12150, t12153, t12154, t12156)
}
