//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 607/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk607<F: Float>(t603: F, t695: F, t598: F, t610: F, t1621: F, t1953: F, t2061: F, t7: F, t226: F, t163: F, t169: F, t616: F, t717: F) -> (F, F, F, F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t4217 = F::cast_from(0.0011033703703703704_f64) * t695 * t603;
    let t4220 = t598 * t610;
    let t4227 = t598 * t1621;
    let t4231 = F::cast_from(1.2833333333333334_f64) * t1953 - F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t2061;
    let t4232 = t4231 * pi;
    let t4233 = t4232 * t7;
    let t4235 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t226 * t4233;
    let t4250 = t169 * t717 * t616 * t163;
    (t4217, t4220, t4227, t4231, t4232, t4233, t4235, t4250)
}
