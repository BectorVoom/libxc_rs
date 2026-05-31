//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1024/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1024<F: Float>(t1325: F, t1326: F, t2005: F, t2961: F, t4804: F, t5266: F, t3794: F, t2954: F, t3518: F, t5250: F, t784: F, t3838: F, t4763: F) -> (F, F, F, F, F) {
    let t11999 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t1325 * t1326 * t2005 * t2961;
    let t12001 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t4804 * t5266;
    let t12003 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t3794 * t5266;
    let t12008 = F::cast_from(64.0_f64) / F::cast_from(81.0_f64) * t1325 * t5250 * t784 * t3518 * t2954;
    let t12010 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t4763 * t3838;
    (t11999, t12001, t12003, t12008, t12010)
}
