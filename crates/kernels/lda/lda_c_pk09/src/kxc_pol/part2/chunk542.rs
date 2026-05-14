//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 542/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk542<F: Float>(t4093: F, t623: F, t896: F, t1063: F, t4361: F, t4365: F, t10: F, t104: F, t125: F, t1092: F, t3148: F, t1106: F, t86: F, t975: F, t143: F, t3557: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4494 = t896 * t4093 * t623;
    let t4497 = t1063 * t4361;
    let t4499 = t1063 * t4365;
    let t4502 = t104 * t125 * t10;
    let t4503 = t1092 * t4502;
    let t4504 = t4503 * t3148;
    let t4512 = t1106 * t4365;
    let t4517 = t86 * t125 * t10;
    let t4518 = t975 * t4517;
    let t4519 = t4518 * t3148;
    let t4528 = 4.178971354861182 * t143 * t3557;
    (t4494, t4497, t4499, t4502, t4504, t4512, t4517, t4519, t4528)
}
