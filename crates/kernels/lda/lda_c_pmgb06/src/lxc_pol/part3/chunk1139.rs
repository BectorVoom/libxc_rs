//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1139/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1139<F: Float>(t12325: F, t13532: F, t1619: F, t2060: F, t3103: F, t9147: F, t9149: F, t9151: F, t9179: F, t9181: F, t9184: F, t9186: F, t9215: F, t9217: F, t9679: F, t9681: F, t9683: F, t9685: F, t9687: F, t9700: F, t9702: F) -> F {
    let t13554 = F::new(0.10666666666666667) * t13532 + F::new(0.09597777777777777) * t9147 + F::new(0.07198333333333333) * t9149 - F::new(0.047988888888888886) * t9179 - F::new(0.03199259259259259) * t9181 + F::new(0.011997222222222222) * t9184 + F::new(0.013330246913580247) * t9186 + F::new(0.11197407407407407) * t9215 - F::new(0.047988888888888886) * t9217 + F::new(0.5038833333333333) * t12325 + F::new(0.044444444444444446) * t9679 - F::new(0.008888888888888889) * t9681 - F::new(0.007407407407407408) * t9683 + F::new(0.0044444444444444444) * t9685 + F::new(0.0019753086419753087) * t9687 - F::new(0.022222222222222223) * t9700 + F::new(0.05925925925925926) * t9702 - F::new(0.07198333333333333) * t9151 - F::new(0.013333333333333334) * t2060 * t1619 * t3103;
    t13554
}
