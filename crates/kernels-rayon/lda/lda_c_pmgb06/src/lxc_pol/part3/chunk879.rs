//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 879/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk879(t1007: f64, t1010: f64, t1004: f64, t1008: f64, t1011: f64, t1012: f64, t1058: f64, t109: f64, t1179: f64, t138: f64, t260: f64, t273: f64, t3738: f64, t3779: f64, t3783: f64, t3784: f64, t3787: f64, t3788: f64, t3792: f64, t3794: f64, t3800: f64, t3804: f64, t3807: f64, t3811: f64, t3818: f64, t3822: f64, t3829: f64, t3842: f64, t409: f64, t660: f64, t668: f64, t675: f64, t682: f64, t683: f64, t8599: f64, t8678: f64, t8681: f64, t8794: f64, t8867: f64, t986: f64, t990: f64) -> f64 {
    let t8930 = t1007 * t1007;
    let t8933 = t1010 * t1010;
    let t8990 = -14.03573669432315_f64 * t3807 * t8599 * t682 + 199645.6030360464_f64 * t260 / t8930 * t8867 / t8933 + 1157.9025239058626_f64 * t3784 * t8867 * t1011 + 91082.60419215256_f64 * t273 * t8678 * t8599 * t8681 - 24828.48620125123_f64 * t260 / t1007 / t990 * t8867 * t3787 + 4.406033529855123_f64 * t138 * t409 * t1008 * t1012 - 141.71548179536398_f64 * t138 * t109 * t3783 * t3788 + 13.218100589565369_f64 * t138 * t109 * t3792 * t3794 - 0.06849333333333334_f64 * t138 * t986 * t3779 + 0.13698666666666667_f64 * t138 * t3822 * t1004 - 0.41096_f64 * t138 * t3829 * t3800 - 0.21309037037037037_f64 * t138 * t1179 * t660 * t668 - 38.025319932552506_f64 * t138 * t109 * t3738 * t3804 - 0.13012297560362088_f64 * t138 * t3818 * t3842 - 0.06747117253521083_f64 * t138 * t1179 * t675 * t683 + 0.04337432520120696_f64 * t138 * t3811 * t1058 + t8794;
    t8990
}
