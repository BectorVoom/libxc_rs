//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 786/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk786<F: Float>(t9353: F, t9363: F, t9375: F, t9390: F, t110: F, t89: F, t1124: F, t2314: F, t1094: F, t1091: F, t121: F, t3141: F, t2362: F, t4502: F, t3148: F, t1101: F, t4411: F, t4413: F, t4421: F, t8595: F, t8597: F, t8600: F, t8602: F, t8604: F, t8606: F, t8608: F, t9056: F, t9060: F, t98: F) -> (F,) {
    let t9392 = t9353 + t9363 + t9375 + t9390;
    let t9393 = t110 * t9392;
    let t9394 = t9393 * t89;
    let t9408 = t2314 * t1124;
    let t9409 = t9408 * t1094;
    let t9410 = t121 * t1091;
    let t9411 = t3141 * t9410;
    let t9414 = t2362 * t4502;
    let t9415 = t9414 * t3148;
    let t9417 = -0.09983749558483038 * t4411 - t4413 / 9.0 + t4421 / 6.0 + t9394 * t98 / 6.0 - 0.02466859483068398 * t8595 - 0.02466859483068398 * t8597 + 0.14975624337724558 * t8600 + 0.29951248675449116 * t8602 + 0.14975624337724558 * t8604 + 0.14975624337724558 * t8606 + 0.29951248675449116 * t8608 + t1101 * t9056 / 6.0 + t1101 * t9060 / 3.0 + t9409 * t9411 / 6.0 + t9415 / 6.0;
    (t9417,)
}
