//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1300/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1300(t13392: f64, t15323: f64, t17070: f64, t13388: f64, t13332: f64, t13337: f64, t17049: f64, t17052: f64, t17054: f64, t17057: f64, t17059: f64, t17061: f64, t17064: f64, t17066: f64) -> (f64, f64, f64) {
    let t17072 = t15323 * t13392 * t17070;
    let t17075 = t15323 * t13388 * t17070;
    let t17077 = -0.19195555555555555_f64 * t17049 + 0.5758666666666666_f64 * t17052 + 0.023994444444444443_f64 * t17054 + 0.09597777777777777_f64 * t17057 + 0.03199259259259259_f64 * t17059 - 0.010664197530864198_f64 * t17061 - 0.2879333333333333_f64 * t17064 - 0.015996296296296297_f64 * t17066 + 0.14396666666666666_f64 * t13332 - 0.06398518518518519_f64 * t13337 + 1.7276_f64 * t17072 - 1.1517333333333333_f64 * t17075;
    (t17072, t17075, t17077)
}
