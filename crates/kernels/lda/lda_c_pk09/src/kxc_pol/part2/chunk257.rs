//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 257/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk257<F: Float>(t1146: F, t8: F, t1137: F, t1139: F, t1143: F, t240: F, t5: F, t252: F) -> (F, F, F) {
    let t1147 = t8 * t1146;
    let t1150 = -F::cast_from(2.0004184593989263_f64) * t1137 - F::cast_from(8.223552159732785_f64) * t1139 + F::cast_from(5.687617677680484_f64) * t5 - F::cast_from(0.2360201854237762_f64) * t1143 - F::cast_from(9.157473255573062e-05_f64) * t240 * t1147;
    let t1151 = t1150 * t252;
    (t1147, t1150, t1151)
}
