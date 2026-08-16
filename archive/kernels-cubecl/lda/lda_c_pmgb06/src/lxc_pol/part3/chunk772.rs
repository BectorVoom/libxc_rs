//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 772/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk772<F: Float>(t1897: F, t4672: F, t439: F, t1901: F, t4650: F, t4668: F, t2010: F, t1420: F, t1902: F, t153: F, t3279: F, t1859: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5242 = t1897 * t4672;
    let t5244 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t439 * t5242;
    let t5245 = t1901 * t4650;
    let t5247 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t439 * t5245;
    let t5248 = t1897 * t4668;
    let t5250 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t2010 * t5248;
    let t5252 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t1420 * t1902;
    let t5253 = t3279 * t153;
    let t5254 = t5253 * t1859;
    (t5242, t5244, t5245, t5247, t5248, t5250, t5252, t5253, t5254)
}
