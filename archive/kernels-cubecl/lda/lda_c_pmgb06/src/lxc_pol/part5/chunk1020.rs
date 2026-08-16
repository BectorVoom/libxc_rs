//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1020/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1020<F: Float>(t19181: F, t19204: F, t1: F, t1981: F, t2871: F, t6516: F, t1420: F, t7563: F, t15256: F, t2095: F, t2563: F, t1887: F, t2606: F) -> (F, F, F, F, F, F) {
    let t19205 = t19181 + t19204;
    let t19209 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1981 * t2871 * t6516 * t1;
    let t19211 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t1420 * t7563;
    let t19215 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t15256;
    let t19217 = t2563 * t2095 / F::cast_from(10.0_f64);
    let t19219 = t1887 * t2606 / F::cast_from(5.0_f64);
    (t19205, t19209, t19211, t19215, t19217, t19219)
}
