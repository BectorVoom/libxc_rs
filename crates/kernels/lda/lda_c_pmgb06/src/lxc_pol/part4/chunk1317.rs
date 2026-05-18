//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1317/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1317<F: Float>(t13332: F, t13337: F, t17049: F, t17052: F, t17054: F, t17057: F, t17059: F, t17061: F, t17064: F, t17066: F, t17072: F, t17075: F) -> F {
    let t17321 = F::new(0.010075555555555556) * t17049 - F::new(0.030226666666666666) * t17052 - F::new(0.0012594444444444445) * t17054 - F::new(0.005037777777777778) * t17057 - F::new(0.0016792592592592592) * t17059 + F::new(0.000559753086419753) * t17061 + F::new(0.015113333333333333) * t17064 + F::new(0.0008396296296296296) * t17066 - F::new(0.007556666666666666) * t13332 + F::new(0.0033585185185185185) * t13337 - F::new(0.09068) * t17072 + F::new(0.06045333333333333) * t17075;
    t17321
}
