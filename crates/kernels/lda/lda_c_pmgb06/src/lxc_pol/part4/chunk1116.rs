//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1116/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1116<F: Float>(t1423: F, t6465: F, t6475: F, t2477: F, t3220: F, t6300: F, t5211: F, t6303: F, t12898: F, t1420: F, t1444: F, t1629: F, t1848: F, t1966: F, t1967: F, t2010: F, t2011: F, t2090: F, t2481: F, t3177: F, t439: F, t4779: F, t5039: F, t5168: F, t6114: F, t6241: F, t6253: F, t9774: F) -> (F,) {
    let t16687 = t1423 * t6465;
    let t16689 = t1423 * t6475;
    let t16697 = t3220 * t2477;
    let t16699 = t1423 * t6300;
    let t16701 = t5211 * t6303;
    let t16720 = -4.0 / 81.0 * t16687 + 32.0 / 243.0 * t16689 - 2.0 / 15.0 * t1848 * t2090 + t9774 / 135.0 - 8.0 / 135.0 * t12898 + 2.0 / 15.0 * t1444 * t6114 + 8.0 / 135.0 * t16697 + 8.0 / 135.0 * t16699 - 4.0 / 27.0 * t16701 + 8.0 / 45.0 * t2010 * t4779 * t2011 + 8.0 / 45.0 * t5168 * t6303 + t3177 * t2481 / 45.0 + 2.0 / 45.0 * t1420 * t6241 + 2.0 / 15.0 * t439 * t1966 * t1967 * t5039 - t439 * t1966 * t6253 * t1629 / 5.0;
    (t16720,)
}
