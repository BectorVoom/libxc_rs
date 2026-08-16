//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1317/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1317(t13332: f64, t13337: f64, t17049: f64, t17052: f64, t17054: f64, t17057: f64, t17059: f64, t17061: f64, t17064: f64, t17066: f64, t17072: f64, t17075: f64) -> f64 {
    let t17321 = 0.010075555555555556_f64 * t17049 - 0.030226666666666666_f64 * t17052 - 0.0012594444444444445_f64 * t17054 - 0.005037777777777778_f64 * t17057 - 0.0016792592592592592_f64 * t17059 + 0.000559753086419753_f64 * t17061 + 0.015113333333333333_f64 * t17064 + 0.0008396296296296296_f64 * t17066 - 0.007556666666666666_f64 * t13332 + 0.0033585185185185185_f64 * t13337 - 0.09068_f64 * t17072 + 0.06045333333333333_f64 * t17075;
    t17321
}
