//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1021/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1021<F: Float>(t21032: F, t1897: F, t19782: F, t2010: F, t1420: F, t7547: F, t439: F, t5253: F, t7485: F, t1901: F, t19758: F, t17859: F, t17861: F, t21013: F, t21016: F, t21021: F, t21026: F, t21028: F) -> (F, F, F, F, F, F) {
    let t21033 = t21032 / 15.0;
    let t21036 = 4.0 / 15.0 * t2010 * t1897 * t19782;
    let t21038 = t1420 * t7547 / 9.0;
    let t21041 = t439 * t5253 * t7485 / 9.0;
    let t21044 = t439 * t1901 * t19758 / 9.0;
    let t21045 = t21013 - t21016 - t21021 - t21026 - t21028 + 4.0 * t17859 + 8.0 * t17861 - t21033 - t21036 + t21038 + t21041 + t21044;
    (t21033, t21036, t21038, t21041, t21044, t21045)
}
