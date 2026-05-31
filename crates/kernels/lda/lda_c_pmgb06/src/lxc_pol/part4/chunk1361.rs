//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1361/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1361<F: Float>(t14017: F, t14019: F, t1916: F, t5194: F, t1972: F, t5333: F, t1920: F, t1594: F, t2570: F, t439: F, t9084: F, t15349: F, t1897: F) -> (F, F, F, F, F, F, F) {
    let t17884 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t14017;
    let t17885 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t14019;
    let t17886 = t5194 * t1916;
    let t17887 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t17886;
    let t17889 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1972 * t5333;
    let t17890 = t5194 * t1920;
    let t17891 = F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t17890;
    let t17895 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t439 * t9084 * t2570 * t1594;
    let t17898 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t439 * t1897 * t15349;
    (t17884, t17885, t17887, t17889, t17891, t17895, t17898)
}
