//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1027/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1027<F: Float>(t13560: F, t2085: F, t2060: F, t848: F, t2082: F, t955: F, t2079: F, t405: F, t4848: F, t4853: F, t4913: F, t4899: F, t103: F, t11997: F, t12563: F, t12568: F, t13407: F, t1576: F, t2923: F, t2932: F, t3358: F, t525: F, t9522: F, t9530: F, t9532: F, t9534: F, t9537: F, t9552: F, t9554: F) -> (F,) {
    let t14162 = t13560 * t2085;
    let t14170 = t2060 * t848;
    let t14181 = t955 * t2082;
    let t14183 = t955 * t2079;
    let t14185 = t405 * t4848;
    let t14187 = t4913 * t4853;
    let t14189 = t405 * t4899;
    let t14198 = 0.03732469135802469 * t13407 + 0.28444444444444444 * t14162 + 0.013333333333333334 * t2060 * t1576 * t2923 - 0.08 * t2060 * t525 * t2932 + 0.019753086419753086 * t14170 + 0.035555555555555556 * t103 * t3358 * t12563 - 0.08 * t2060 * t1576 * t12568 + 0.24 * t2060 * t525 * t11997 + 0.044444444444444446 * t14181 - 0.007407407407407408 * t14183 - 0.02666666666666667 * t14185 + 0.3466666666666667 * t14187 + 0.0044444444444444444 * t14189 - 0.047988888888888886 * t9522 - 0.047988888888888886 * t9530 - 0.03199259259259259 * t9532 + 0.011997222222222222 * t9534 + 0.013330246913580247 * t9537 + 0.11197407407407407 * t9552 + 0.07198333333333333 * t9554;
    (t14198,)
}
