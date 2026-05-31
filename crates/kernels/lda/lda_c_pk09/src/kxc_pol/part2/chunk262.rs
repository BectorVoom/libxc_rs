//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 262/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk262<F: Float>(t1179: F, t266: F, t42: F, t560: F, t561: F, t1168: F) -> (F, F, F, F, F, F) {
    let t1180 = t266 * t1179;
    let t1181 = F::cast_from(2.0970850588349075_f64) * t42;
    let t1182 = F::cast_from(7.140848730573871_f64) * t560;
    let t1183 = F::cast_from(8.043763671738963_f64) * t561;
    let t1184 = F::cast_from(3.0_f64) * t1168;
    let t1185 = t1181 - t1182 + t1183 - t1184;
    (t1180, t1181, t1182, t1183, t1184, t1185)
}
