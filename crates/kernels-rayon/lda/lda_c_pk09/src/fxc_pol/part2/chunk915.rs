//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 915/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk915(t4917: f64, t9723: f64, t1207: f64, t258: f64, t9699: f64, t4926: f64, t630: f64, t2467: f64, t1195: f64, t1204: f64, t2455: f64, t2468: f64, t4882: f64, t9680: f64, t9717: f64, t9720: f64) -> (f64, f64, f64) {
    let t9724 = t9723 * t4917;
    let t9727 = t258 * t1207;
    let t9728 = t9699 * t9727;
    let t9731 = t4926 * t630;
    let t9732 = t2467 * t9731;
    let t9735 = t9680 * t1204 + t2455 * t630 * t1207 + t9717 - t9720 + 1.28_f64 * t4882 * t2468 - 1.28_f64 * t1195 * t9724 - 2.56_f64 * t1195 * t9728 - 1.28_f64 * t1195 * t9732;
    (t9727, t9731, t9735)
}
