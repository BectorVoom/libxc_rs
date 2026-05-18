//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 880/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk880<F: Float>(t1003: F, t1009: F, t1011: F, t1054: F, t1055: F, t1061: F, t269: F, t2786: F, t282: F, t30: F, t3758: F, t3778: F, t3784: F, t3787: F, t3793: F, t3803: F, t3807: F, t3851: F, t3858: F, t666: F, t668: F, t676: F, t681: F, t682: F, t683: F, t8719: F, t8771: F, t8814: F, t8822: F, t8830: F, t8837: F, t8841: F, t957: F, t964: F, t967: F, t992: F, t993: F, t994: F) -> F {
    let t9033 = F::new(0.5848223622634646) * t676 * t8719 * t682 - t8814 - t8822 - t8830 + F::new(36.0) * t1009 * t994 * t1003 - F::new(1157.9025239058626) * t3793 * t3851 * t993 + t8837 - t8841 + F::new(69.26343642272586) * t1061 * t3758 * t967 * t681 - F::new(4.678578898107717) * t1054 * t683 * t3758 + F::new(6152.411314929844) * t3803 * t8771 * t964 + F::new(21.053605041484726) * t1061 * t1055 * t957 - F::new(623.3709278045327) * t3807 * t3858 * t964 - F::new(0.005520940648395062) * t30 * t2786 * t269 - F::new(0.0018989649058080863) * t30 * t2786 * t282 + F::new(12414.243100625616) * t3784 * t1003 * t3787 * t993 + F::new(128.6558359895403) * t1009 * t3778 * t1011 * t666 - F::new(8.0) * t992 * t668 * t3778;
    t9033
}
