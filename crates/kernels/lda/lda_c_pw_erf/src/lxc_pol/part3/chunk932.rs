//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 932/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk932<F: Float>(t331: F, t3620: F, t3611: F, t4233: F, t598: F, t226: F, t4606: F, t5021: F, t7: F, t1397: F, t4073: F, t1472: F, t3748: F) -> (F, F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t10250 = t331 * t3620;
    let t10252 = t331 * t3611;
    let t10278 = t598 * t4233;
    let t10286 = F::new(4.0) / F::new(3.0) * t226 * (-F::cast_from(4.277777777777778_f64) * t4606 + F::new(220.0) / F::new(81.0) * t5021) * pi * t7;
    let t10294 = t4073 * t1397;
    let t10296 = t1472 * t3748;
    (t10250, t10252, t10278, t10286, t10294, t10296)
}
