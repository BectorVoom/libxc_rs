//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1178/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1178(t132: f64, t435: f64, t7802: f64, t14212: f64, t17982: f64, t17984: f64, t17991: f64, t17993: f64, t17996: f64, t18002: f64, t18004: f64, t18006: f64, t18008: f64, t18010: f64) -> f64 {
    let t21242 = t132 * t435 * t7802;
    let t21254 = -t21242 / 45.0_f64 + t14212 + 2.0_f64 / 27.0_f64 * t17982 + 2.0_f64 / 27.0_f64 * t17984 + 4.0_f64 / 45.0_f64 * t17991 + 4.0_f64 / 45.0_f64 * t17993 + 8.0_f64 / 45.0_f64 * t17996 + 2.0_f64 / 45.0_f64 * t18002 + 2.0_f64 / 45.0_f64 * t18004 + 2.0_f64 / 45.0_f64 * t18006 - 4.0_f64 / 45.0_f64 * t18008 - 4.0_f64 / 45.0_f64 * t18010;
    t21254
}
