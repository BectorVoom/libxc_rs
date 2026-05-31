//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 587/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk587<F: Float>(t1026: F, t1067: F, t4165: F, t87: F, t3163: F, t1098: F, t3498: F, t609: F, t650: F, t96: F, t839: F, t106: F, t4281: F) -> (F, F, F, F, F) {
    let t4451 = t1026 * t1067;
    let t4457 = t87 * t4165;
    let t4459 = t4457 * t3163 / F::cast_from(3.0_f64);
    let t4461 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1098 * t3498;
    let t4474 = t96 * t650 * t609;
    let t4475 = t839 * t4474;
    let t4478 = F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t106 * t4281;
    (t4451, t4459, t4461, t4475, t4478)
}
