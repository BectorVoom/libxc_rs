//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1275/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1275<F: Float>(t1: F, t397: F, t6011: F, t8464: F, t8482: F, t8486: F, t8495: F, t8499: F, t14435: F, t8510: F, t8469: F, t8473: F, t8477: F, t8481: F, t8491: F, t8493: F, t8505: F, t8509: F, t8516: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18981 = t6011 * t1 * t397;
    let t18982 = 0.0003662311007350632 * t18981;
    let t18983 = 0.0011393856467313078 * t8464;
    let t18984 = 7.017868076946245 * t8482;
    let t18985 = 103.89453539625518 * t8486;
    let t18986 = 24.0 * t8495;
    let t18987 = 12.0 * t8499;
    let t18988 = 120.0 * t14435;
    let t18989 = 64.0 * t8510;
    let t18990 = -t18982 - t18983 + t8469 + t8473 - t8477 - t8481 + t18984 - t18985 + t8491 + t8493 - t18986 + t18987 - t8505 + t8509 + t18988 + t18989 + t8516;
    (t18982, t18983, t18984, t18985, t18986, t18987, t18988, t18989, t18990)
}
