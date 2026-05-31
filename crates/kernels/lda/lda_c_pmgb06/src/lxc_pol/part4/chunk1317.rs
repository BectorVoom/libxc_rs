//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1317/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1317<F: Float>(t13332: F, t13337: F, t17049: F, t17052: F, t17054: F, t17057: F, t17059: F, t17061: F, t17064: F, t17066: F, t17072: F, t17075: F) -> F {
    let t17321 = F::cast_from(0.010075555555555556_f64) * t17049 - F::cast_from(0.030226666666666666_f64) * t17052 - F::cast_from(0.0012594444444444445_f64) * t17054 - F::cast_from(0.005037777777777778_f64) * t17057 - F::cast_from(0.0016792592592592592_f64) * t17059 + F::cast_from(0.000559753086419753_f64) * t17061 + F::cast_from(0.015113333333333333_f64) * t17064 + F::cast_from(0.0008396296296296296_f64) * t17066 - F::cast_from(0.007556666666666666_f64) * t13332 + F::cast_from(0.0033585185185185185_f64) * t13337 - F::cast_from(0.09068_f64) * t17072 + F::cast_from(0.06045333333333333_f64) * t17075;
    t17321
}
