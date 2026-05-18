//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 834/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk834<F: Float>(t121: F, t8493: F, t633: F, t8488: F, t707: F, t2288: F, t4668: F, t823: F, t3262: F, t7731: F, t8338: F, t890: F) -> (F, F, F, F, F) {
    let t8494 = t121 * t8493;
    let t8497 = t8488 * t633;
    let t8498 = t707 * t8497;
    let t8501 = t2288 * t4668;
    let t8502 = t8501 * t823;
    let t8503 = t121 * t8502;
    let t8506 = t3262 * t7731;
    let t8508 = t890 * t8338;
    (t8494, t8498, t8503, t8506, t8508)
}
