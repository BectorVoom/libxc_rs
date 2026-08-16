//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 582/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk582(t409: f64, t660: f64, t1008: f64, t109: f64, t1004: f64, t1009: f64, t1012: f64, t1050: f64, t1058: f64, t1062: f64, t138: f64, t3672: f64, t3678: f64, t3700: f64, t3779: f64, t3784: f64, t3788: f64, t3793: f64, t3794: f64, t3797: f64, t3800: f64, t3803: f64, t3804: f64, t3807: f64, t3808: f64, t3811: f64, t3818: f64, t661: f64, t668: f64, t676: f64, t683: f64, t986: f64) -> (f64, f64, f64) {
    let t3822 = t409 * t660;
    let t3829 = t109 * t1008;
    let t3833 = 1.0_f64 * t661 * t3779 - t3672 + 2069.040516770936_f64 * t3784 * t3788 - 192.98375398431043_f64 * t3793 * t3794 + 0.5848223622634646_f64 * t676 * t3797 + t3678 - t3700 + 6.0_f64 * t1009 * t3800 + 1025.4018858216407_f64 * t3803 * t3804 - 103.89515463408878_f64 * t3807 * t3808 + 0.02168716260060348_f64 * t138 * t3811 * t683 - 0.01626537195045261_f64 * t138 * t1050 * t1058 - 0.4815973313767657_f64 * t138 * t3818 * t1062 + 0.06849333333333334_f64 * t138 * t3822 * t668 - 0.05137_f64 * t138 * t986 * t1004 - 1.652262573695671_f64 * t138 * t3829 * t1012;
    (t3822, t3829, t3833)
}
