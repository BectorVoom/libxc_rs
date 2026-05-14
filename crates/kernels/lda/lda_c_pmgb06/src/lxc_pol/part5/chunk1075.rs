//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1075/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1075<F: Float>(t20403: F, t20406: F, t20409: F, t20412: F, t20415: F, t20420: F, t20423: F, t20428: F, t20431: F, t20435: F, t20436: F, t20438: F, t13088: F, t20440: F, t20442: F, t20445: F, t20449: F, t20451: F, t20452: F, t20454: F, t20456: F, t20460: F, t20463: F) -> (F, F) {
    let t21991 = -t20403 + t20406 + t20409 - t20412 + t20415 - t20420 - t20423 + t20428 - t20431 + t20435 - t20436 - t20438;
    let t21995 = -t20440 + t20442 + t20445 + t20449 + t20451 + t13088 - t20452 - t20454 - t20456 - t20460 + t20463;
    (t21991, t21995)
}
