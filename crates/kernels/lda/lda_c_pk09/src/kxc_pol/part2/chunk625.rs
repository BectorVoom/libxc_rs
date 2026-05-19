//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 625/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk625<F: Float>(t5161: F, t1226: F, t5153: F, t1240: F, t623: F, t73: F) -> (F, F, F, F) {
    let t5162 = F::cast_from(2.6666666666666665_f64) * t5161;
    let t5163 = t1226 * t5153;
    let t5164 = t1240 * t623;
    let t5166 = t5163 * t73 * t5164;
    (t5162, t5163, t5164, t5166)
}
