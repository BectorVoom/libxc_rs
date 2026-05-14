//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 646/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk646<F: Float>(t7136: F, t93: F, t6501: F, t6505: F, t6522: F, t6319: F, t6325: F, t6547: F, t6464: F, t1672: F, t2071: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7137 = t93 * t7136;
    let t7149 = 1.4770435158815312 * t6501;
    let t7150 = 1.4770435158815312 * t6505;
    let t7154 = 1.9693913545087083 * t6522;
    let t7158 = 0.2946275542389858 * t6319;
    let t7165 = 0.1964183694926572 * t6325;
    let t7166 = 0.16411594620905903 * t6547;
    let t7171 = 0.06547278983088574 * t6464;
    let t7183 = 0.7661514025603425 * t6501;
    let t7184 = 0.7661514025603425 * t6505;
    let t7188 = 1.02153520341379 * t6522;
    let t7192 = 0.15282509383508946 * t6319;
    let t7199 = 0.10188339589005964 * t6325;
    let t7200 = 0.08512793361781583 * t6547;
    let t7205 = 0.033961131963353215 * t6464;
    let t7221 = t2071 * t1672;
    (t7137, t7149, t7150, t7154, t7158, t7165, t7166, t7171, t7183, t7184, t7188, t7192, t7199, t7200, t7205, t7221)
}
