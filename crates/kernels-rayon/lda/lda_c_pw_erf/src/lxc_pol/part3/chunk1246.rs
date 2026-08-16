//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1246/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1246(t14639: f64, t1686: f64, t1852: f64, t10: f64, t14634: f64, t14656: f64, t14658: f64, t14660: f64, t14781: f64, t14783: f64, t14787: f64, t14796: f64, t14797: f64, t14799: f64, t3222: f64, t426: f64, t5571: f64) -> f64 {
    let t14802 = t1686 * t1852 * t14639;
    let t14803 = 5.87616_f64 * t14802;
    let t14804 = -0.97936_f64 * t14781 - 88.1424_f64 * t14783 * t14658 + t14656 - t14660 + 44.0712_f64 * t14787 + 30.0_f64 * t426 * t10 * t5571 * t3222 - t426 * t14634 / 2.0_f64 - t14796 + 1.95872_f64 * t14797 - 8.81424_f64 * t14799 + t14803;
    t14804
}
