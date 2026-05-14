//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1039/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1039<F: Float>(t11983: F, t1318: F, t1403: F, t549: F, t833: F, t4039: F, t795: F, t1: F, t3: F, t4713: F, t604: F, t1635: F, t4537: F, t1639: F, t20: F, t5794: F) -> (F, F, F, F, F) {
    let t14088 = 24.0 / 5.0 * t1318 * t11983 * t833 * t1403 * t549;
    let t14089 = t795 * t4039;
    let t14090 = 16.0 / 405.0 * t14089;
    let t14093 = t4713 * t1 * t3 * t604;
    let t14095 = t4537 * t1635;
    let t14096 = 0.6492624817418906 * t14095;
    let t14098 = t5794 * t20 * t1639;
    (t14088, t14090, t14093, t14096, t14098)
}
