//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1122/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1122<F: Float>(t10030: F, t7752: F, t18184: F, t2010: F, t3974: F, t1949: F, t2478: F, t4574: F, t1944: F, t5165: F, t12314: F, t6725: F) -> (F, F, F, F, F) {
    let t20836 = t10030 * t7752;
    let t20837 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t20836;
    let t20840 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t3974 * t18184 * t2010;
    let t20844 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t3974 * t4574 * t2478 * t1949;
    let t20848 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t3974 * t5165 * t2478 * t1944;
    let t20850 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t12314 * t6725;
    (t20837, t20840, t20844, t20848, t20850)
}
