//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1182/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1182<F: Float>(t103: F, t11997: F, t12563: F, t12568: F, t13407: F, t14162: F, t14170: F, t14181: F, t14183: F, t14185: F, t14187: F, t14189: F, t1576: F, t2060: F, t2923: F, t2932: F, t3358: F, t525: F, t9522: F, t9530: F, t9532: F, t9534: F, t9537: F, t9552: F, t9554: F) -> F {
    let t14198 = F::cast_from(0.03732469135802469_f64) * t13407 + F::cast_from(0.28444444444444444_f64) * t14162 + F::cast_from(0.013333333333333334_f64) * t2060 * t1576 * t2923 - F::new(0.08) * t2060 * t525 * t2932 + F::cast_from(0.019753086419753086_f64) * t14170 + F::cast_from(0.035555555555555556_f64) * t103 * t3358 * t12563 - F::new(0.08) * t2060 * t1576 * t12568 + F::new(0.24) * t2060 * t525 * t11997 + F::cast_from(0.044444444444444446_f64) * t14181 - F::cast_from(0.007407407407407408_f64) * t14183 - F::cast_from(0.02666666666666667_f64) * t14185 + F::cast_from(0.3466666666666667_f64) * t14187 + F::cast_from(0.0044444444444444444_f64) * t14189 - F::cast_from(0.047988888888888886_f64) * t9522 - F::cast_from(0.047988888888888886_f64) * t9530 - F::cast_from(0.03199259259259259_f64) * t9532 + F::cast_from(0.011997222222222222_f64) * t9534 + F::cast_from(0.013330246913580247_f64) * t9537 + F::cast_from(0.11197407407407407_f64) * t9552 + F::cast_from(0.07198333333333333_f64) * t9554;
    t14198
}
