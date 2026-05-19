//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 716/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk716<F: Float>(t6547: F, t6464: F, t132: F, t2069: F, t93: F, t6501: F, t6505: F, t6522: F, t6319: F, t6325: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7124 = F::cast_from(0.17025586723563146_f64) * t6547;
    let t7129 = F::cast_from(0.06792226392670653_f64) * t6464;
    let t7136 = t132 * t2069;
    let t7137 = t93 * t7136;
    let t7149 = F::cast_from(1.4770435158815312_f64) * t6501;
    let t7150 = F::cast_from(1.4770435158815312_f64) * t6505;
    let t7154 = F::cast_from(1.9693913545087083_f64) * t6522;
    let t7158 = F::cast_from(0.2946275542389858_f64) * t6319;
    let t7165 = F::cast_from(0.1964183694926572_f64) * t6325;
    let t7166 = F::cast_from(0.16411594620905903_f64) * t6547;
    let t7171 = F::cast_from(0.06547278983088574_f64) * t6464;
    let t7183 = F::cast_from(0.7661514025603425_f64) * t6501;
    let t7184 = F::cast_from(0.7661514025603425_f64) * t6505;
    (t7124, t7129, t7137, t7149, t7150, t7154, t7158, t7165, t7166, t7171, t7183, t7184)
}
