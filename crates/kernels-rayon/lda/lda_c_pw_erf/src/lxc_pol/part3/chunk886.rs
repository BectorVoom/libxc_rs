//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 886/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk886(t2818: f64, t2820: f64, t2916: f64, t2855: f64, t684: f64, t2859: f64, t1138: f64, t147: f64, t8363: f64, t2783: f64, t688: f64, t692: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8812 = 1.8276876377896586e-05_f64 * t2818 * t2916 * t2820;
    let t8813 = t684 * t2855;
    let t8816 = t684 * t2859;
    let t8821 = 6.701521338562081e-05_f64 * t8363 * t147 * t1138 * t2820;
    let t8822 = t2783 * t688;
    let t8825 = 0.7805426614091894_f64 * t2783 * t692;
    (t8812, t8813, t8816, t8821, t8822, t8825)
}
