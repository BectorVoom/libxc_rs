//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 577/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk577<F: Float>(t4093: F, t609: F, t891: F, t4092: F, t1025: F, t119: F, t10: F, t1024: F, t88: F, t1106: F, t3498: F, t1006: F, t1062: F) -> (F, F, F, F, F, F) {
    let t4095 = t891 * t4093 * t609;
    let t4096 = t4092 * t4095;
    let t4098 = t1025 * t119;
    let t4104 = t1024 * t88 * t10;
    let t4109 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1106 * t3498;
    let t4110 = t1006 * t1062;
    (t4095, t4096, t4098, t4104, t4109, t4110)
}
