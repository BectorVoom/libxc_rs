//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 953/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk953(t10003: f64, t10005: f64, t10011: f64, t10013: f64, t10017: f64, t10021: f64, t10023: f64, t10025: f64, t2559: f64, t2587: f64, t4983: f64, t5047: f64, t5071: f64, t5684: f64, t5689: f64, t5691: f64, t5693: f64, t5694: f64, t5696: f64, t5701: f64, t5703: f64, t5706: f64, t5778: f64, t5933: f64, t9777: f64) -> f64 {
    let t10036 = t10003 * t10005 / 3.0_f64 + t5778 * t2587 / 6.0_f64 + t10011 * t10013 / 6.0_f64 + t10017 / 6.0_f64 - t10021 / 6.0_f64 - t10023 * t10025 / 3.0_f64 - t5684 / 6.0_f64 + t5689 / 6.0_f64 + t5691 - t5693 - t5694 - 0.10237773105191754_f64 * t5047 + t5696 - 0.03412591035063918_f64 * t5071 + t2559 * t5933 / 12.0_f64 - 0.04991874779241519_f64 * t9777 + t5701 - t5703 + 0.02466859483068398_f64 * t4983 + t5706;
    t10036
}
