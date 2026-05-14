//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 805/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk805<F: Float>(t2468: F, t4875: F, t2467: F, t4910: F, t4821: F, t1179: F, t2146: F, t4917: F, t1207: F, t258: F, t9699: F, t4926: F, t630: F, t1195: F, t1204: F, t2455: F, t4882: F, t9680: F) -> (F, F, F) {
    let t9717 = 1.28 * t4875 * t2468;
    let t9718 = t2467 * t4910;
    let t9720 = 1.28 * t4821 * t9718;
    let t9723 = t1179 * t2146;
    let t9724 = t9723 * t4917;
    let t9727 = t258 * t1207;
    let t9728 = t9699 * t9727;
    let t9731 = t4926 * t630;
    let t9732 = t2467 * t9731;
    let t9735 = t9680 * t1204 + t2455 * t630 * t1207 + t9717 - t9720 + 1.28 * t4882 * t2468 - 1.28 * t1195 * t9724 - 2.56 * t1195 * t9728 - 1.28 * t1195 * t9732;
    (t9727, t9731, t9735)
}
