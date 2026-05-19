//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 991/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk991<F: Float>(t5043: F, t5056: F, t5158: F, t5166: F, t5177: F, t5193: F, t6082: F, t6098: F, t9623: F, t9631: F, t9635: F, t9742: F, t9750: F, t9948: F, t9952: F, t9956: F, t9959: F) -> F {
    let t10657 = F::cast_from(3.2084841915276807_f64) * t9948 + F::cast_from(3.2084841915276807_f64) * t9952 - F::cast_from(3.2084841915276807_f64) * t9956 + F::cast_from(2.1389894610184537_f64) * t9959 - F::new(0.64) * t9623 - F::cast_from(0.21333333333333335_f64) * t9631 - F::new(0.64) * t9635 - F::new(0.64) * t9742 - F::new(0.64) * t9750 - F::new(0.64) * t5043 - F::cast_from(0.21333333333333335_f64) * t5056 + t6082 - F::cast_from(2.1389894610184537_f64) * t5177 + F::cast_from(2.1389894610184537_f64) * t5193 + t6098 - F::cast_from(6.416968383055361_f64) * t5158 + F::cast_from(6.416968383055361_f64) * t5166;
    t10657
}
