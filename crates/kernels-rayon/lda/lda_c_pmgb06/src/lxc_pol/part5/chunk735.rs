//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 735/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk735(t161: f64, t6843: f64, t2592: f64, t436: f64, t2600: f64, t489: f64, t2901: f64, t3350: f64, t4876: f64, t4879: f64, t4896: f64, t4898: f64, t4909: f64, t4911: f64, t4914: f64, t4916: f64, t6803: f64, t6806: f64, t6809: f64, t6814: f64, t6817: f64, t6822: f64, t6825: f64) -> (f64, f64, f64, f64, f64) {
    let t6844 = t161 * t6843;
    let t6846 = t2592 * t436;
    let t6851 = t489 * t2600;
    let t6852 = t161 * t6851;
    let t6868 = -0.03999074074074074_f64 * t6803 + 0.14396666666666666_f64 * t6806 + 0.09597777777777777_f64 * t6809 - 0.21595_f64 * t6814 - 0.2879333333333333_f64 * t6817 - 0.023994444444444443_f64 * t6822 + 0.07198333333333333_f64 * t6825 - 0.047988888888888886_f64 * t4876 + t4879 - t4896 + t4898 - 0.014814814814814815_f64 * t4909 - 0.03199259259259259_f64 * t4911 - 0.017777777777777778_f64 * t4914 - 0.047988888888888886_f64 * t4916 - 0.007407407407407408_f64 * t3350 - 0.015996296296296297_f64 * t2901;
    (t6844, t6846, t6851, t6852, t6868)
}
