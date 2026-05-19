//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 953/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk953<F: Float>(t10003: F, t10005: F, t10011: F, t10013: F, t10017: F, t10021: F, t10023: F, t10025: F, t2559: F, t2587: F, t4983: F, t5047: F, t5071: F, t5684: F, t5689: F, t5691: F, t5693: F, t5694: F, t5696: F, t5701: F, t5703: F, t5706: F, t5778: F, t5933: F, t9777: F) -> F {
    let t10036 = t10003 * t10005 / F::new(3.0) + t5778 * t2587 / F::new(6.0) + t10011 * t10013 / F::new(6.0) + t10017 / F::new(6.0) - t10021 / F::new(6.0) - t10023 * t10025 / F::new(3.0) - t5684 / F::new(6.0) + t5689 / F::new(6.0) + t5691 - t5693 - t5694 - F::cast_from(0.10237773105191754_f64) * t5047 + t5696 - F::cast_from(0.03412591035063918_f64) * t5071 + t2559 * t5933 / F::new(12.0) - F::cast_from(0.04991874779241519_f64) * t9777 + t5701 - t5703 + F::cast_from(0.02466859483068398_f64) * t4983 + t5706;
    t10036
}
