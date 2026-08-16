//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1096/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1096<F: Float>(t10954: F, t10962: F, t10966: F, t11062: F, t11070: F, t11556: F, t11559: F, t11563: F, t11566: F, t11574: F, t6327: F, t6519: F, t6527: F, t7371: F, t7378: F, t7379: F, t7384: F) -> F {
    let t12072 = F::cast_from(12.5_f64) * t6527 - F::cast_from(12.5_f64) * t6519 - F::cast_from(6.25_f64) * t11556 + F::cast_from(6.25_f64) * t11559 - F::cast_from(0.41556487541815906_f64) * t10962 + F::cast_from(4.166666666666667_f64) * t11563 - F::cast_from(4.166666666666667_f64) * t11566 - F::cast_from(1.2466946262544771_f64) * t11070 - F::cast_from(1.2466946262544771_f64) * t10954 - F::cast_from(1.2466946262544771_f64) * t10966 - F::cast_from(1.2466946262544771_f64) * t11062 + F::cast_from(6.25_f64) * t11574 + F::cast_from(1.2466946262544771_f64) * t6327 + t7371 + t7378 - t7379 - t7384;
    t12072
}
