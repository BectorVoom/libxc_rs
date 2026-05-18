//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 992/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk992<F: Float>(t10641: F, t10657: F, t1278: F, t306: F, t403: F, t9602: F, t1397: F, t2516: F, t93: F, t1240: F, t5603: F, t2474: F, t741: F) -> (F, F, F, F, F) {
    let t10658 = t10641 + t10657;
    let t10659 = t10658 * t1278;
    let t10660 = t10659 * t306;
    let t10669 = t403 * t9602;
    let t10678 = t2516 * t1397;
    let t10679 = t93 * t10678;
    let t10682 = t2516 * t1240;
    let t10683 = t93 * t10682;
    let t10684 = t5603 * t10683;
    let t10686 = t741 * t2474;
    (t10660, t10669, t10679, t10684, t10686)
}
