//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1300/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1300<F: Float>(t13392: F, t15323: F, t17070: F, t13388: F, t13332: F, t13337: F, t17049: F, t17052: F, t17054: F, t17057: F, t17059: F, t17061: F, t17064: F, t17066: F) -> (F, F, F) {
    let t17072 = t15323 * t13392 * t17070;
    let t17075 = t15323 * t13388 * t17070;
    let t17077 = -F::cast_from(0.19195555555555555_f64) * t17049 + F::cast_from(0.5758666666666666_f64) * t17052 + F::cast_from(0.023994444444444443_f64) * t17054 + F::cast_from(0.09597777777777777_f64) * t17057 + F::cast_from(0.03199259259259259_f64) * t17059 - F::cast_from(0.010664197530864198_f64) * t17061 - F::cast_from(0.2879333333333333_f64) * t17064 - F::cast_from(0.015996296296296297_f64) * t17066 + F::cast_from(0.14396666666666666_f64) * t13332 - F::cast_from(0.06398518518518519_f64) * t13337 + F::cast_from(1.7276_f64) * t17072 - F::cast_from(1.1517333333333333_f64) * t17075;
    (t17072, t17075, t17077)
}
