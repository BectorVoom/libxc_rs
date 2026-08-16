//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1067/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1067<F: Float>(t10954: F, t10962: F, t10966: F, t11062: F, t11070: F, t11556: F, t11559: F, t11563: F, t11566: F, t11574: F, t6327: F, t6519: F, t6527: F, t7158: F, t7165: F, t7166: F, t7171: F) -> F {
    let t11577 = F::cast_from(2.9540870317630623_f64) * t6527 - F::cast_from(2.9540870317630623_f64) * t6519 - F::cast_from(1.4770435158815312_f64) * t11556 + F::cast_from(1.4770435158815312_f64) * t11559 - F::cast_from(0.0982091847463286_f64) * t10962 + F::cast_from(0.9846956772543541_f64) * t11563 - F::cast_from(0.9846956772543541_f64) * t11566 - F::cast_from(0.2946275542389858_f64) * t11070 - F::cast_from(0.2946275542389858_f64) * t10954 - F::cast_from(0.2946275542389858_f64) * t10966 - F::cast_from(0.2946275542389858_f64) * t11062 + F::cast_from(1.4770435158815312_f64) * t11574 + F::cast_from(0.2946275542389858_f64) * t6327 + t7158 + t7165 - t7166 - t7171;
    t11577
}
