//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 644/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk644<F: Float>(t409: F, t660: F, t1008: F, t109: F, t1004: F, t1009: F, t1012: F, t1050: F, t1058: F, t1062: F, t138: F, t3672: F, t3678: F, t3700: F, t3779: F, t3784: F, t3788: F, t3793: F, t3794: F, t3797: F, t3800: F, t3803: F, t3804: F, t3807: F, t3808: F, t3811: F, t3818: F, t661: F, t668: F, t676: F, t683: F, t986: F) -> (F, F, F) {
    let t3822 = t409 * t660;
    let t3829 = t109 * t1008;
    let t3833 = F::new(1.0) * t661 * t3779 - t3672 + F::new(2069.040516770936) * t3784 * t3788 - F::new(192.98375398431043) * t3793 * t3794 + F::new(0.5848223622634646) * t676 * t3797 + t3678 - t3700 + F::new(6.0) * t1009 * t3800 + F::new(1025.4018858216407) * t3803 * t3804 - F::new(103.89515463408878) * t3807 * t3808 + F::new(0.02168716260060348) * t138 * t3811 * t683 - F::new(0.01626537195045261) * t138 * t1050 * t1058 - F::new(0.4815973313767657) * t138 * t3818 * t1062 + F::new(0.06849333333333334) * t138 * t3822 * t668 - F::new(0.05137) * t138 * t986 * t1004 - F::new(1.652262573695671) * t138 * t3829 * t1012;
    (t3822, t3829, t3833)
}
