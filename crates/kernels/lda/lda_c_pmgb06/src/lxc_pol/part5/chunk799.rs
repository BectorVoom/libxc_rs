//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 799/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk799<F: Float>(t1007: F, t1010: F, t1004: F, t1008: F, t1011: F, t1012: F, t1058: F, t109: F, t1179: F, t138: F, t260: F, t273: F, t3738: F, t3779: F, t3783: F, t3784: F, t3787: F, t3788: F, t3792: F, t3794: F, t3800: F, t3804: F, t3807: F, t3811: F, t3818: F, t3822: F, t3829: F, t3842: F, t409: F, t660: F, t668: F, t675: F, t682: F, t683: F, t8599: F, t8678: F, t8681: F, t8794: F, t8867: F, t986: F, t990: F) -> (F,) {
    let t8930 = t1007 * t1007;
    let t8933 = t1010 * t1010;
    let t8990 = -14.03573669432315 * t3807 * t8599 * t682 + 199645.6030360464 * t260 / t8930 * t8867 / t8933 + 1157.9025239058626 * t3784 * t8867 * t1011 + 91082.60419215256 * t273 * t8678 * t8599 * t8681 - 24828.48620125123 * t260 / t1007 / t990 * t8867 * t3787 + 4.406033529855123 * t138 * t409 * t1008 * t1012 - 141.71548179536398 * t138 * t109 * t3783 * t3788 + 13.218100589565369 * t138 * t109 * t3792 * t3794 - 0.06849333333333334 * t138 * t986 * t3779 + 0.13698666666666667 * t138 * t3822 * t1004 - 0.41096 * t138 * t3829 * t3800 - 0.21309037037037037 * t138 * t1179 * t660 * t668 - 38.025319932552506 * t138 * t109 * t3738 * t3804 - 0.13012297560362088 * t138 * t3818 * t3842 - 0.06747117253521083 * t138 * t1179 * t675 * t683 + 0.04337432520120696 * t138 * t3811 * t1058 + t8794;
    (t8990,)
}
