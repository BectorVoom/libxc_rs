//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 778/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk778<F: Float>(t1058: F, t8720: F, t1062: F, t2379: F, t721: F, t1007: F, t1011: F, t2337: F, t2341: F, t4021: F, t4353: F, t4354: F, t4362: F, t4366: F, t8508: F, t8510: F, t8512: F, t8517: F, t8519: F, t8521: F, t98: F) -> (F,) {
    let t9220 = t8720 * t1058;
    let t9223 = t2379 * t1062;
    let t9224 = t9223 * t721;
    let t9237 = -t1007 * t2341 / 6.0 - t9220 * t98 / 6.0 + t9224 / 6.0 + t2337 * t1011 / 6.0 + t4353 + t4354 - 0.016445729887122652 * t4021 + t4362 / 6.0 + t4366 / 6.0 + 0.037002892246025966 * t8508 - 0.02466859483068398 * t8510 - 0.02466859483068398 * t8512 + 0.02466859483068398 * t8517 + 0.02466859483068398 * t8519 + 0.14975624337724558 * t8521;
    (t9237,)
}
