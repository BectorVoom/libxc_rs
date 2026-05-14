//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1019/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1019<F: Float>(t2002: F, t6465: F, t6475: F, t6275: F, t6478: F, t20981: F, t20984: F, t20987: F, t20992: F, t20995: F, t20999: F, t21001: F, t21003: F, t12592: F, t19377: F, t493: F) -> (F, F, F, F, F) {
    let t21005 = t2002 * t6465 / 9.0;
    let t21007 = 8.0 / 27.0 * t2002 * t6475;
    let t21009 = 4.0 / 9.0 * t6275 * t6478;
    let t21010 = -t20981 - t20984 - t20987 - t20992 - t20995 - t20999 - t21001 + t21003 - t21005 + t21007 + t21009;
    let t21013 = 88.0 / 243.0 * t493 * t12592 * t19377;
    (t21005, t21007, t21009, t21010, t21013)
}
