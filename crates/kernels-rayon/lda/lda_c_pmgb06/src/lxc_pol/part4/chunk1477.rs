//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1477/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1477(t10964: f64, t10967: f64, t10980: f64, t10984: f64, t10990: f64, t14773: f64, t14776: f64, t14831: f64, t14866: f64, t15081: f64, t15124: f64, t18444: f64, t18486: f64, t18892: f64, t19045: f64, t19055: f64, t19069: f64, t2: f64, t328: f64, t8028: f64) -> f64 {
    let tv4rho42 = 0.9480012043054112_f64 * t10967 - 2.530897186465939_f64 * t10980 + 0.8215265768013333_f64 * t14773 + 0.5476843845342222_f64 * t10984 + 2.0_f64 * t14776 + t2 * (t14831 + t14866 + t15081 + t15124 + t18444 + t18486 + t18892 + t19045) * t328 + 0.6327242966164848_f64 * t10964 + 2.530897186465939_f64 * t10990 + 0.13692109613355555_f64 * t19055 + 0.13692109613355555_f64 * t8028 + t19069;
    tv4rho42
}
