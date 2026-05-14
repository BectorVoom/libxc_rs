//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 771/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk771<F: Float>(t1076: F, t1095: F, t1101: F, t2355: F, t3138: F, t3195: F, t3201: F, t4275: F, t4278: F, t8892: F, t8895: F, t9037: F, t9040: F, t9043: F, t9046: F, t9050: F, t9054: F, t9056: F, t9060: F, t98: F) -> (F,) {
    let t9069 = 0.14975624337724558 * t3195 + 0.14975624337724558 * t3201 - t9037 * t98 / 6.0 + t9040 / 6.0 + t1101 * t9043 / 3.0 + t1101 * t9046 / 6.0 + t9050 / 6.0 + t1095 * t8892 / 6.0 - t9054 / 6.0 - t1076 * t9056 / 6.0 - t1076 * t9060 / 3.0 + t1095 * t8895 / 6.0 + t2355 * t3138 / 6.0 + t4275 / 9.0 - t4278 / 54.0;
    (t9069,)
}
