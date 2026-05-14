//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 995/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk995<F: Float>(t1446: F, t4834: F, t5234: F, t5238: F, t3476: F, t5146: F, t197: F, t3892: F, t3518: F, t3556: F, t795: F, t2120: F, t3550: F, t3553: F, t1234: F, t1982: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12015 = t1446 * t4834;
    let t12017 = t1446 * t5234;
    let t12019 = t1446 * t5238;
    let t12025 = t5146 * t3476;
    let t12030 = t3892 * t197;
    let t12031 = t12030 * t3518;
    let t12044 = t795 * t3556;
    let t12046 = t2120 * t3550;
    let t12050 = t795 * t3553;
    let t12052 = t1982 * t1234;
    (t12015, t12017, t12019, t12025, t12030, t12031, t12044, t12046, t12050, t12052)
}
