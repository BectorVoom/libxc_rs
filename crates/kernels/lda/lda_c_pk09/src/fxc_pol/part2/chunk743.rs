//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 743/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk743<F: Float>(t169: F, t7704: F, t849: F, t2149: F, t4710: F, t707: F, t2288: F, t825: F, t609: F, t121: F, t4037: F, t623: F, t633: F, t4668: F, t823: F, t3262: F, t7731: F) -> (F, F, F, F, F, F, F) {
    let t8475 = t849 * t169 * t7704;
    let t8484 = t4710 * t2149;
    let t8485 = t707 * t8484;
    let t8488 = t2288 * t825;
    let t8489 = t8488 * t609;
    let t8490 = t121 * t8489;
    let t8491 = t4037 * t8490;
    let t8493 = t8488 * t623;
    let t8494 = t121 * t8493;
    let t8497 = t8488 * t633;
    let t8498 = t707 * t8497;
    let t8501 = t2288 * t4668;
    let t8502 = t8501 * t823;
    let t8503 = t121 * t8502;
    let t8506 = t3262 * t7731;
    (t8475, t8485, t8491, t8494, t8498, t8503, t8506)
}
