//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1179/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1179<F: Float>(t13372: F, t13343: F, t13345: F, t13347: F, t13350: F, t13353: F, t13356: F, t13359: F, t13362: F, t13365: F, t13368: F, t13370: F, t13374: F, t13376: F, t13379: F, t9938: F, t9940: F, t9954: F, t9956: F, t9958: F) -> F {
    let t14127 = F::new(0.03199259259259259) * t13372;
    let t14136 = -F::new(0.8638) * t13343 + F::new(0.023994444444444443) * t13345 + F::new(0.03999074074074074) * t13347 - F::new(0.023994444444444443) * t13350 - F::new(0.10664197530864197) * t13353 + F::new(1.2957) * t13356 + F::new(0.14396666666666666) * t13359 + F::new(0.23994444444444443) * t13362 + F::new(0.07198333333333333) * t13365 - F::new(0.4319) * t13368 + F::new(0.09597777777777777) * t13370 - t14127 - F::new(0.07198333333333333) * t13374 + F::new(1.5836333333333332) * t13376 - F::new(0.14396666666666666) * t13379 + F::new(0.05925925925925926) * t9938 + F::new(0.02666666666666667) * t9940 + F::new(0.044444444444444446) * t9954 - F::new(0.022222222222222223) * t9956 - F::new(0.007407407407407408) * t9958;
    t14136
}
