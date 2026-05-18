//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 716/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk716<F: Float>(t6547: F, t6464: F, t132: F, t2069: F, t93: F, t6501: F, t6505: F, t6522: F, t6319: F, t6325: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7124 = F::new(0.17025586723563146) * t6547;
    let t7129 = F::new(0.06792226392670653) * t6464;
    let t7136 = t132 * t2069;
    let t7137 = t93 * t7136;
    let t7149 = F::new(1.4770435158815312) * t6501;
    let t7150 = F::new(1.4770435158815312) * t6505;
    let t7154 = F::new(1.9693913545087083) * t6522;
    let t7158 = F::new(0.2946275542389858) * t6319;
    let t7165 = F::new(0.1964183694926572) * t6325;
    let t7166 = F::new(0.16411594620905903) * t6547;
    let t7171 = F::new(0.06547278983088574) * t6464;
    let t7183 = F::new(0.7661514025603425) * t6501;
    let t7184 = F::new(0.7661514025603425) * t6505;
    (t7124, t7129, t7137, t7149, t7150, t7154, t7158, t7165, t7166, t7171, t7183, t7184)
}
