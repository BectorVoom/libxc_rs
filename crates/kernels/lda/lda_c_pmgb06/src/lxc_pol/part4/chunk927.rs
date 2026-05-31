//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 927/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk927<F: Float>(t161: F, t6843: F, t2592: F, t436: F, t2600: F, t489: F, t2901: F, t3350: F, t4876: F, t4879: F, t4896: F, t4898: F, t4909: F, t4911: F, t4914: F, t4916: F, t6803: F, t6806: F, t6809: F, t6814: F, t6817: F, t6822: F, t6825: F) -> (F, F, F, F, F) {
    let t6844 = t161 * t6843;
    let t6846 = t2592 * t436;
    let t6851 = t489 * t2600;
    let t6852 = t161 * t6851;
    let t6868 = -F::cast_from(0.03999074074074074_f64) * t6803 + F::cast_from(0.14396666666666666_f64) * t6806 + F::cast_from(0.09597777777777777_f64) * t6809 - F::cast_from(0.21595_f64) * t6814 - F::cast_from(0.2879333333333333_f64) * t6817 - F::cast_from(0.023994444444444443_f64) * t6822 + F::cast_from(0.07198333333333333_f64) * t6825 - F::cast_from(0.047988888888888886_f64) * t4876 + t4879 - t4896 + t4898 - F::cast_from(0.014814814814814815_f64) * t4909 - F::cast_from(0.03199259259259259_f64) * t4911 - F::cast_from(0.017777777777777778_f64) * t4914 - F::cast_from(0.047988888888888886_f64) * t4916 - F::cast_from(0.007407407407407408_f64) * t3350 - F::cast_from(0.015996296296296297_f64) * t2901;
    (t6844, t6846, t6851, t6852, t6868)
}
