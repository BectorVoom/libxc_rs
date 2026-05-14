//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1062/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1062<F: Float>(t19529: F, t19530: F, t19532: F, t19533: F, t19535: F, t19536: F, t19538: F, t19539: F, t19540: F, t19541: F, t19542: F, t19543: F, t19599: F, t19602: F, t19605: F, t19608: F, t19613: F, t19617: F, t19621: F, t19626: F, t19630: F, t19634: F, t19639: F, t19642: F) -> (F, F) {
    let t21912 = t19529 - t19530 + t19532 + t19533 + t19535 + t19536 + t19538 - t19539 - t19540 - t19541 - t19542 - t19543;
    let t21915 = t19599 - t19602 + t19605 - t19608 + t19613 + t19617 + t19621 + t19626 - t19630 - t19634 - t19639 + t19642;
    (t21912, t21915)
}
